mod terminal;
mod signal;
mod tokenizer;
mod line;

mod candidate;
mod history;

use std::io::{self, Write};
use std::fs::OpenOptions;

use crossterm::{event::{KeyModifiers, KeyCode}};
use terminal::Terminal;

use line::Line;
use history::History;
pub use signal::Signal;
// use candidate::Candidate;

pub struct LineEditor {
    prompt: &'static str,
    terminal: Terminal,
    history: History,

    line_count: usize,
    overflow_left : usize,
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

            line_count: 1,
            overflow_left : 0,
            overflow_right: 0,
            visible_area_width: term_width - prompt.len() - 2
            //                                              ^ this is initial label width
        }
    }

    fn log(&self, content: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).open("log.txt")?;

        file.write(content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    fn clear_line(&mut self) -> io::Result<()> {
        self.terminal.cursor.move_to_pos(self.prompt.len())?;
        self.terminal.clear_after_cursor();
        Ok(())
    }

    fn refresh(&mut self, line: &Line) {
        let total_width = self.prompt.len() + line.len() + line.label_width;
        let term_width = self.terminal.width();

        self.overflow_left = if total_width > term_width {
            total_width - term_width
        } else { 0 };
    }
    fn render(&mut self, line: &Line) -> io::Result<()> {
        self.terminal.cursor.hide()?;
        self.clear_line()?;

        let mut offset = self.overflow_left;
        let mut remain_space = self.visible_area_width;
        for token in &line.tokens {
            if remain_space == 0 {
                break;
            }

            if offset > 0 {
                if offset >= token.len() {
                    offset -= token.len();
                } else {
                    // offset < token.len()
                    let actual_print_len = token.len() - offset;

                    // when a token is going to be overflow
                    if actual_print_len > remain_space {
                        self.terminal.print(&token.content[offset..offset+remain_space], token.type__);
                        break;
                    }

                    remain_space -= token.len() - offset;
                    self.terminal.print(&token.content[offset..], token.type__);
                    offset = 0;
                }
            } else {
                if remain_space >= token.len() {
                    remain_space -= token.len();
                    self.terminal.print(&token.content, token.type__);
                } else {
                    self.terminal.print(&token.content[..remain_space], token.type__);
                    remain_space = 0;
                }
            }
        }

        print!("{}", &line.label);
        self.terminal.flush()?;
        self.terminal.cursor.show()
    }

    fn scroll_left(&mut self) {
        if self.overflow_left > 0 {
            self.overflow_left  -= 1;
            self.overflow_right += 1;
        }
    }
    fn scroll_right(&mut self) {
        if self.overflow_right > 0 {
            self.overflow_right -= 1;
            self.overflow_left  += 1;
        }
    }

    fn insert_edit(&mut self, ch: char, line: &mut Line) -> io::Result<()> {
        let insert_pos =
            self.terminal.cursor_col()? - self.prompt.len() + self.overflow_left;
        line.insert(insert_pos, ch);

        if line.len() - 1 >= self.visible_area_width {
            self.overflow_left += 1;
        } else {
            self.terminal.cursor.right(1)?;
        }
        Ok(())
    }
    fn remove_edit(&mut self, line: &mut Line) -> io::Result<()> {
        let cursor_pos = self.terminal.cursor_col()?;
        if cursor_pos == 0 {
            return Ok(())
        }

        let mut remove_pos = cursor_pos - self.prompt.len() + self.overflow_left;
        if self.overflow_left > 0 {
            remove_pos -= 1;
        }
        line.remove(remove_pos);

        if self.overflow_left > 0 {
            self.overflow_left -= 1;
        } else
        if self.overflow_right > 0 {
            self.overflow_right -= 1;
        }
        Ok(())
    }

    pub fn readline(&mut self) -> io::Result<Signal> {
        print!("{}", self.prompt);
        self.terminal.flush()?;

        let mut line_content = String::new();
        let mut line = Line::new(
            &mut line_content,
            self.line_count,
        );

        let result = loop {
            let Some(key) = self.terminal.get_key() else {
                continue;
            };
            // ctrl + c -> Interrupt
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                println!("\nKeyboard Interrupt");
                break Signal::Interrupt;
            }

            let cursor_pos = self.terminal.cursor_col()?;
            let term_width = self.terminal.width();
            let prompt_len = self.prompt.len();

            // self.log(&format!("line content: {}, cursor pos: {}, terminal width: {}, visible_width: {}", line.content, cursor_pos.to_string(), term_width, self.visible_area_width))?;
            // visible left & right end
            let is_at_left_end  = cursor_pos == prompt_len;
            let is_at_right_end = cursor_pos == term_width - line.label_width;
            // virtual line left & right end
            let is_at_line_start = is_at_left_end && (self.overflow_left == 0);
            let is_at_line_end =
                ((cursor_pos - self.prompt.len()) == line.len())
                || (is_at_right_end && self.overflow_right == 0);

            match key.code {
                // use with history
                KeyCode::Up    => {
                    // if let Some(last_line) = self.history.previous() {
                    //     // let slice = last_line.as_str();
                    //     line.reset_with(&last_line[..]);
                    // }
                },
                KeyCode::Down  => todo!(),
                
                KeyCode::Left  => {
                    if is_at_line_start {
                        continue;
                    }

                    if is_at_left_end && self.overflow_left > 0 {
                        // println!("SCROLL_LEFT"); // LOG
                        self.scroll_left();
                    } else {
                        self.terminal.cursor.left(1)?;
                        continue;
                    }
                },
                KeyCode::Right => {
                    if is_at_line_end {
                        continue;
                    }

                    if is_at_right_end {
                        self.scroll_right();
                    } else {
                        self.terminal.cursor.right(1)?;
                        continue;
                    }
                },

                KeyCode::Enter => {
                    // println!("test: {}", self.terminal.width() - self.prompt.len() - line.label_width);
                    // println!("overflow_left: {}", self.overflow_left);
                    // println!("\nis_left_end  : {is_at_left_end}, is_right_end: {is_at_right_end}");
                    // println!("\nis_line_start: {is_at_line_start}, is_line_end: {is_at_line_end}");

                    self.line_count += 1;
                    self.overflow_left  = 0;
                    self.overflow_right = 0;
                    self.terminal.new_line();
                    break Signal::NewLine(line_content);
                },
                KeyCode::Tab => {
                    // if is_at_right_end {
                    //     self.display_hint()?;
                    // }
                    // continue;
                },
                KeyCode::Backspace => {
                    if is_at_line_start {
                        continue;
                    }

                    if self.overflow_left == 0 {
                        self.terminal.cursor.left(1)?;
                    }

                    if is_at_line_end {
                        line.pop();
                        self.refresh(&line);
                    } else {
                        self.remove_edit(&mut line)?;
                    }
                },

                KeyCode::Char(ch) => {
                    if !ch.is_ascii() {
                        // avoid Non-ASCII character
                        self.terminal.new_line();
                        break Signal::NonASCII;
                    }

                    if is_at_line_end {
                        if !is_at_right_end {
                            self.terminal.cursor.right(1)?;
                        }

                        line.push(ch);
                        self.refresh(&line);
                    } else {
                        self.insert_edit(ch, &mut line)?;
                    }

                    // if is_at_right_end {
                    //     line.push(ch);
                    // } else {
                    //     self.insert_edit(&mut line, ch)?;
                    //     continue;
                    // }
                },
                _ => {}
            }
            self.terminal.cursor.save_pos()?;
            self.render(&line)?;
            self.terminal.cursor.restore_pos()?;
        };
        Ok(result)
    }
}