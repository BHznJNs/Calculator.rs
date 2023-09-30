mod analyzer;
mod line;
mod signal;

mod candidate;
mod history;

use std::{io, mem};

use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::Stylize,
};

use crate::public::run_time::scope::Scope;
use crate::utils::{cursor::Cursor, print_line, terminal::Terminal};

use super::text_area::TextArea;

pub use signal::Signal;
use history::EditorHistory;
use line::EditorLine;

pub struct LineEditor {
    current_line: EditorLine,
    history: EditorHistory,
}

impl LineEditor {
    const PROMPT: &'static str = "> ";

    pub fn new() -> io::Result<Self> {
        let initial_line = EditorLine::new(Self::PROMPT.len(), 1);

        return Ok(Self {
            current_line: initial_line,
            history: EditorHistory::new(),
        });
    }

    fn init_render(&self) -> io::Result<()> {
        let line_label = self.current_line.index.to_string();

        Cursor::move_to_col(Terminal::width() - line_label.len())?;
        print!("{}", line_label.bold().black().on_white());
        Cursor::move_to_col(0)?;
        print!("{}", Self::PROMPT);

        Terminal::flush()?;
        return Ok(());
    }

    pub fn readline(&mut self, scope: &mut Scope) -> io::Result<Signal> {
        self.init_render()?;

        let result = loop {
            let Some(key) = Terminal::get_key() else {
                continue;
            };

            if key.modifiers == KeyModifiers::CONTROL {
                if key.code == KeyCode::Char('c') || key.code == KeyCode::Char('d') {
                    break Signal::Interrupt;
                } else {
                    continue;
                }
            }

            match key.code {
                KeyCode::Up | KeyCode::Down => {
                    let target_item = match key.code {
                        KeyCode::Up => {
                            if !self.history.use_history {
                                let current_content = self.current_line.content().to_owned();
                                self.history.set_cached(current_content);
                            }
                            self.history.previous()
                        }
                        KeyCode::Down => self.history.next(),
                        _ => unreachable!()
                    };
                    if let Some(str) = target_item {
                        self.current_line.set_content(str)?;
                    }
                }
                KeyCode::Enter => {
                    print_line("");

                    let current_line_index = self.current_line.index;
                    let new_line = EditorLine::new(Self::PROMPT.len(), current_line_index + 1);
                    let line = mem::replace(&mut self.current_line, new_line);
                    self.history.append(line.content().to_owned());
                    break Signal::NewLine(line.content().to_owned());
                }

                // avoid Non-ASCII characters
                KeyCode::Char(ch) if !ch.is_ascii() => break Signal::NonASCII,
                k if TextArea::is_editing_key(k) => self.current_line.edit(k)?,
                _ => {}
            }
            Terminal::flush()?;
        };
        return Ok(result);
    }
}
