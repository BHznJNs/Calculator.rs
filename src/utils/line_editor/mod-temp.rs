mod terminal;
mod signal;
mod candidate;
mod tokenizer;
mod line;

use std::io;

use crossterm::{event::{KeyModifiers, KeyCode}};
use terminal::{Terminal, TextType};

use self::{line::Line, tokenizer::TokenVec};

use super::completer::Completer;

pub use signal::Signal;
use candidate::Candidate;

pub struct LineEditor {
    prompt: String,
    terminal: Terminal,
    // history: History,
    candidate: Candidate,

    current_word: String,
}

impl LineEditor {
    pub fn new(prompt: String) -> Self {
        let prompt_len = prompt.len();
        LineEditor {
            prompt,
            terminal: Terminal::new(prompt_len),
            // history: History::new(),
            candidate: Candidate::new(),

            current_word: String::new(),
        }
    }

    fn is_at_right_end(&self, line: &Line) -> io::Result<bool> {
        Ok(self.terminal.cursor_col()? == line.len())
    }

    fn display_prompt(&mut self) -> io::Result<()> {
        print!("{}", self.prompt);
        self.terminal.flush()
    }
    fn display_hint(&mut self) -> io::Result<()> {
        if let Some(hint) = self.candidate.next() {
            self.terminal.cursor.hide()?;
            self.terminal.clear_after_cursor()?;

            self.terminal.print(hint, TextType::Hint);
            self.terminal.flush()?;

            self.terminal.cursor.left(hint.len() as u16)?;
            self.terminal.cursor.show()?;
        }
        Ok(())
    }

    fn render(&mut self, tokens: &TokenVec) -> io::Result<()> {
        self.terminal.cursor.hide()?;
        self.terminal.clear_line()?;

        // line.push('\0');
        // let tokens = tokenize(line);
        // line.pop();

        for token in tokens {
            token.output(&mut self.terminal);
        }
        self.terminal.flush()?;
        self.terminal.cursor.show()
    }
    fn insert_edit(&mut self, line: &mut Line, ch: char) -> io::Result<()> {
        // let insert_pos = self.terminal.cursor_col()?;
        // line.insert(insert_pos, ch);

        // self.terminal.cursor.save_pos()?;
        // self.render(line)?;
        // self.terminal.cursor.restore_pos()?;

        // self.terminal.cursor.right(1)
        todo!()
    }
    fn complete(&mut self, line: &mut String) -> io::Result<()> {
        if let Some(hint) = self.candidate.current_hint() {
            *line += hint;
            self.terminal.print(hint, TextType::Variable);
            self.terminal.flush()?;
        }
        Ok(())
    }

    pub fn read(&mut self, completer: &mut Completer) -> io::Result<Signal> {
        let mut line_content = String::new();
        let mut line = Line::new(&mut line_content);
        self.display_prompt()?;

        let result =
        loop {
            let Some(key) = self.terminal.get_key() else {
                continue;
            };

            // ctrl + c -> Interrupt
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                println!("\nKeyboard Interrupt");
                break Signal::Interrupt;
            }

            let is_at_right_end = self.is_at_right_end(&line)?;

            match key.code {
                KeyCode::Left  =>
                    self.terminal.cursor.left(1)?,
                KeyCode::Right => {
                    if is_at_right_end {
                        // self.complete(&mut line)?;
                        self.candidate.set(completer.complete(&self.current_word));
                    } else {
                        self.terminal.cursor.right(1)?
                    }
                },
                // use with history
                // KeyCode::Up    => todo!(),
                // KeyCode::Down  => todo!(),

                KeyCode::Enter => {
                    // println!("Tokens: {:#?}", tokenize(&line)); // LOG
                    self.terminal.new_line();
                    break Signal::NewLine(line_content);
                },
                KeyCode::Tab => {
                    if is_at_right_end {
                        self.display_hint()?;
                    }
                    continue;
                },
                KeyCode::Backspace => {
                    if is_at_right_end {
                        line.pop();
                    } else {
                        todo!()
                    }

                    self.terminal.back()?;
                },

                KeyCode::Char(ch) => {
                    if !ch.is_ascii() {
                        self.terminal.new_line();
                        break Signal::NonASCII;
                    }

                    if is_at_right_end {
                        line.push(ch);
                        self.render(&line.tokens)?;
                    } else {
                        self.insert_edit(&mut line, ch)?;
                    }
                },
                _ => {}
            }
        };
        Ok(result)
    }
}