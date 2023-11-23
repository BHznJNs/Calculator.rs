use std::io;

use crossterm::{event::KeyCode, style::Stylize};

use crate::{
    public::run_time::scope::Scope,
    utils::{
        cursor::Cursor,
        editor::{direction::Direction, text_area::TextArea, tokenizer::TokenSequence},
        number_bit_count,
    },
};

use super::{analyzer::analyze, candidate::Candidate};

pub struct EditorLine {
    pub index: usize,
    text_area: TextArea<TokenSequence>,
    candidates: Candidate,

    // the extra overflow that is used to show the
    // hint completely
    extra_overflow: usize,
}

impl EditorLine {
    pub fn new(prompt_width: usize, line_index: usize) -> Self {
        let line_label_width = number_bit_count(line_index) + 1;
        return Self {
            index: line_index,
            text_area: TextArea::new(prompt_width, line_label_width),
            candidates: Candidate::new(),
            extra_overflow: 0,
        };
    }

    #[inline]
    pub fn is_editing_key(key: KeyCode) -> bool {
        TextArea::is_editing_key(key) || key == KeyCode::Tab || key == KeyCode::BackTab
    }

    #[inline]
    fn restore_overflow(&mut self) {
        self.text_area.overflow_left -= self.extra_overflow;
        self.extra_overflow = 0;
    }

    

    pub fn edit(&mut self, key: KeyCode, scope: &Scope) -> io::Result<()> {
        let text_area = &mut self.text_area;
        match key {
            KeyCode::Backspace => {
                text_area.delete_char(false)?;
                self.restore_overflow();
                self.text_area.render()?;
            }
            KeyCode::Tab | KeyCode::BackTab if self.candidates.current().is_some() => {
                match key {
                    KeyCode::Tab => self.candidates.next(),
                    KeyCode::BackTab => self.candidates.previous(),
                    _ => unreachable!(),
                };
                self.restore_overflow();
                self.show_hint()?;
            }

            KeyCode::Left => {
                if self.candidates.current().is_some() {
                    // when have hint to show, hide it
                    self.hide_hint()?;
                } else {
                    text_area.move_cursor_horizontal(Direction::Left, true)?;
                }
            }
            KeyCode::Right => {
                if !text_area.state_right()?.is_at_content_end {
                    text_area.move_cursor_horizontal(Direction::Right, true)?;
                } else {
                    self.complete_hint()?;
                }
            }
            KeyCode::Char(ch) => {
                text_area.insert_char(ch, false)?;
                if text_area.state_right()?.is_at_content_end {
                    let tokens = text_area.tokens().unwrap();
                    let Some(candidates) = analyze(tokens, scope) else {
                        text_area.render()?;
                        return Ok(());
                    };
                    self.candidates.set(candidates);
                    self.show_hint()?;
                } else {
                    text_area.render()?;
                }
            }
            _ => unreachable!(),
        }
        return Ok(());
    }

    #[inline]
    pub fn jump_to_word_edge(&mut self, dir: Direction) -> io::Result<()> {
        self.text_area.jump_to_word_edge(dir, true)
    }

    pub fn set_content(&mut self, new_content: &str) -> io::Result<()> {
        self.text_area.set_content(new_content);
        self.text_area.move_cursor_to_end(false)?;
        self.text_area.render()?;
        return Ok(());
    }

    #[inline]
    pub fn content(&self) -> &str {
        self.text_area.content()
    }
}

// hint operations
impl EditorLine {
    fn show_hint(&mut self) -> io::Result<()> {
        let visible_area_width = self.text_area.visible_area_width();
        let content_width = self.content().len();
        let Some(hint) = self.candidates.current() else {
            self.text_area.render()?;
            return Ok(());
        };

        if content_width + hint.len() > visible_area_width {
            let new_overflow = (content_width + hint.len()) - visible_area_width;
            let current_overflow = &mut self.text_area.overflow_left;
            self.extra_overflow = new_overflow - *current_overflow;
            *current_overflow += self.extra_overflow;
            Cursor::left(self.extra_overflow)?;
        }
        self.text_area.render()?;
        print!("{}", hint.as_str().dim());
        Cursor::left(hint.len())?;
        return Ok(());
    }
    fn hide_hint(&mut self) -> io::Result<()> {
        self.restore_overflow();
        self.candidates.clear();
        let text_area = &mut self.text_area;
        text_area.move_cursor_to_end(false)?;
        text_area.move_cursor_horizontal(Direction::Left, false)?;
        text_area.render()?;
        return Ok(());
    }
    fn complete_hint(&mut self) -> io::Result<()> {
        self.restore_overflow();
        let Some(hint) = self.candidates.current() else {
            return Ok(());
        };
        let text_area = &mut self.text_area;
        text_area.push_str(hint);
        text_area.move_cursor_to_end(false)?;
        text_area.render()?;
        self.candidates.clear();
        return Ok(());
    }
}
