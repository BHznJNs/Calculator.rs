mod line;
mod signal;
mod state;
mod terminal;
mod tokenizer;

mod candidate;
mod history;

use std::{io, ops::Range, mem};

use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::Stylize,
};

use history::History;
use line::Line;
pub use signal::Signal;
use state::LineState;
use terminal::Terminal;

use crate::utils::output::print_line;
use crate::{public::env::ENV_OPTION, utils::line_editor::tokenizer::Token};
// use candidate::Candidate;

// output something into file
// this function is used to debug.
// fn log(content: &str) -> io::Result<()> {
//     File::create("log.txt")?;
//     let mut file = OpenOptions::new().write(true).open("log.txt")?;
//     file.write(content.as_bytes())?;
//     file.flush()?;
//     Ok(())
// }

pub struct LineEditor {
    prompt: &'static str,
    terminal: Terminal,
    history: History,
    is_at: LineState,

    line_count: usize,
    overflow_left: usize,
    overflow_right: usize,
    visible_area_width: usize,
}

impl LineEditor {
    pub fn new(prompt: &'static str) -> Self {
        let terminal = Terminal::new();
        let term_width = terminal.width();

        LineEditor {
            prompt,
            terminal,
            history: History::new(),
            is_at: LineState::new(),

            line_count: 1,
            overflow_left: 0,
            overflow_right: 0,
            visible_area_width: term_width - prompt.len() - 2,
        }
    }

    fn back_operate(&mut self, line: &mut Line) -> io::Result<()> {
        if self.overflow_left == 0 {
            self.terminal.cursor.left(1)?;
        }

        if self.is_at.line_end {
            line.pop();
            if line.len() > self.visible_area_width {
                self.overflow_left = line.len() - self.visible_area_width;
            } else {
                self.overflow_left = 0;
            };
        } else {
            self.remove_edit(line)?;
        }
        Ok(())
    }

    #[inline]
    fn move_cursor_to_prompt(&mut self) -> io::Result<()> {
        self.terminal.cursor.move_to_col(self.prompt.len())
    }
    #[inline]
    fn clear_line(&mut self) -> io::Result<()> {
        self.move_cursor_to_prompt()?;
        self.terminal.clear_after_cursor();
        Ok(())
    }

    // recompute the states
    fn refresh(&mut self, line: &Line) -> io::Result<()> {
        let cursor_pos = self.terminal.cursor_col()?;
        let term_width = self.terminal.width();
        let prompt_len = self.prompt.len();

        // refresh `self.visible_area_width`
        self.visible_area_width = term_width - prompt_len - 2;

        // visible left & right end
        self.is_at.left_end = cursor_pos == prompt_len;
        self.is_at.right_end = cursor_pos == term_width - line.label_width;

        // virtual line left & right end
        self.is_at.line_start = self.is_at.left_end && (self.overflow_left == 0);
        self.is_at.line_end = ((cursor_pos - prompt_len) == line.len())
            || (self.is_at.right_end && self.overflow_right == 0);

        Ok(())
    }
    fn render(&mut self, line: &Line) -> io::Result<()> {
        #[inline]
        fn buffer_extend_colored(
            buffer: &mut String,
            is_history: bool,
            token: &Token,
            range: Range<usize>,
        ) {
            if unsafe { ENV_OPTION.support_ansi } {
                let mut colored = token.colored(range);
                // if is history, line text will be darken
                if is_history {
                    colored = colored.dim();
                }

                buffer.extend(colored.to_string().chars());
            } else {
                *buffer += &token.content;
            }
        }

        self.terminal.cursor.hide()?;
        self.clear_line()?;

        let mut offset = self.overflow_left;
        let mut remain_space = self.visible_area_width;
        let mut buffer = String::new();
        for token in &line.tokens {
            if remain_space == 0 {
                break;
            }

            if offset > 0 {
                if offset >= token.len() {
                    // token is out of visible area
                    offset -= token.len();
                } else {
                    // token has part in unvisible area
                    let actual_print_len = token.len() - offset;

                    // when a token is going to be overflow left side and right side
                    if actual_print_len > remain_space {
                        // print middle part of this token
                        buffer_extend_colored(
                            &mut buffer,
                            line.is_history,
                            token,
                            offset..offset + remain_space,
                        );
                        break;
                    }

                    remain_space -= token.len() - offset;
                    buffer_extend_colored(&mut buffer, line.is_history, token, offset..token.len());
                    offset = 0;
                }
            } else {
                if remain_space >= token.len() {
                    remain_space -= token.len();
                    buffer_extend_colored(&mut buffer, line.is_history, token, 0..token.len());
                } else {
                    buffer_extend_colored(&mut buffer, line.is_history, token, 0..remain_space);
                    remain_space = 0;
                }
            }
        }

        print!("{}{}", buffer, &line.label);
        self.terminal.flush()?;
        self.terminal.cursor.show()
    }

    // --- --- --- --- --- ---

    fn scroll_left(&mut self) {
        if self.overflow_left > 0 {
            self.overflow_left -= 1;
            self.overflow_right += 1;
        }
    }
    fn scroll_right(&mut self) {
        if self.overflow_right > 0 {
            self.overflow_right -= 1;
            self.overflow_left += 1;
        }
    }

    // --- --- --- --- --- ---

    fn insert_edit(&mut self, ch: char, line: &mut Line) -> io::Result<()> {
        let insert_pos = self.terminal.cursor_col()? - self.prompt.len() + self.overflow_left;
        let is_inserted = line.insert(insert_pos, ch);

        if is_inserted {
            if line.len() - 1 >= self.visible_area_width {
                self.overflow_left += 1;
            } else {
                self.terminal.cursor.right(1)?;
            }
        }
        Ok(())
    }
    fn remove_edit(&mut self, line: &mut Line) -> io::Result<()> {
        let cursor_pos = self.terminal.cursor_col()?;
        if cursor_pos == 0 {
            return Ok(());
        }

        let mut remove_pos = cursor_pos - self.prompt.len() + self.overflow_left;
        if self.overflow_left > 0 {
            remove_pos -= 1;
            self.overflow_left -= 1;
        } else if self.overflow_right > 0 {
            self.overflow_right -= 1;
        }

        line.remove(remove_pos);
        Ok(())
    }

    // --- --- --- --- --- ---

    pub fn readline(&mut self) -> io::Result<Signal> {
        print!("{}", self.prompt);
        self.terminal.flush()?;

        // let mut line_content = String::new();
        let mut line = Line::new(self.line_count);

        let result = loop {
            let Some(key) = self.terminal.get_key() else {
                continue;
            };
            // ctrl + c -> Interrupt
            if key.modifiers == KeyModifiers::CONTROL
                && (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('d'))
            {
                print_line(&mut self.terminal.stdout, "\nKeyboard Interrupt");
                break Signal::Interrupt;
            }

            self.refresh(&line)?;

            // control to display history
            match key.code {
                KeyCode::Up => {
                    if let Some(last_line) = self.history.previous() {
                        self.move_cursor_to_prompt()?;
                        line.use_history(last_line);
                    }
                }

                KeyCode::Down => {
                    if let Some(next_line) = self.history.next() {
                        self.move_cursor_to_prompt()?;
                        line.use_history(next_line);
                    } else {
                        line.reset();
                    }
                }

                KeyCode::Tab => {
                    if let Some(new_content) = self.history.get_current() {
                        if new_content.len() > self.visible_area_width {
                            self.overflow_left = 0;
                            self.overflow_right = new_content.len() - self.visible_area_width;
                        }
                        self.history.reset_index();
                        line.reset_with(new_content);
                    }
                }

                // else: do nothing and continue execute
                _ => {}
            }

            // when displaying history content, disable editing.
            if !line.is_history {                
                match key.code {
                    KeyCode::Left => {
                        if self.is_at.line_start {
                            continue;
                        }

                        if self.is_at.left_end {
                            self.scroll_left();
                        } else {
                            self.terminal.cursor.left(1)?;
                            continue; // skip rerender
                        }
                    }
                    KeyCode::Right => {
                        if self.is_at.line_end {
                            // TODO: complete
                            continue;
                        }

                        if self.is_at.right_end {
                            self.scroll_right();
                        } else {
                            self.terminal.cursor.right(1)?;
                            continue; // skip rerender
                        }
                    }

                    KeyCode::Enter => {
                        self.line_count += 1;
                        self.overflow_left = 0;
                        self.overflow_right = 0;

                        print_line(&mut self.terminal.stdout, "");
                        let line_content = mem::replace(&mut line.content, String::new());

                        self.history.append(line_content.clone());
                        break Signal::NewLine(line_content);
                    }
                    KeyCode::Tab => {
                        if self.is_at.line_end {}
                        // continue;
                    }
                    KeyCode::Backspace => {
                        if self.is_at.line_start {
                            continue;
                        }
                        self.back_operate(&mut line)?;
                    }

                    KeyCode::Char(ch) => {
                        if !ch.is_ascii() {
                            // avoid Non-ASCII character
                            print_line(&mut self.terminal.stdout, "");
                            break Signal::NonASCII;
                        }

                        let is_allowed_char = Line::is_allowed_char(ch);
                        if self.is_at.line_end && is_allowed_char {
                            line.push(ch);

                            if !self.is_at.right_end {
                                self.terminal.cursor.right(1)?;
                            }
                            if line.len() > self.visible_area_width {
                                self.overflow_left = line.len() - self.visible_area_width;
                            } else {
                                self.overflow_left = 0;
                            };
                        } else {
                            self.insert_edit(ch, &mut line)?;
                        }
                    }
                    _ => {}
                }
            }

            self.terminal.cursor.save_pos()?;
            self.render(&line)?;
            self.terminal.cursor.restore_pos()?;
        };
        Ok(result)
    }
}
