use std::io;

use crossterm::event::KeyCode;

use crate::utils::{
    editor::{direction::Direction, text_area::TextArea, tokenizer::TokenSequence},
    number_bit_count,
};

pub struct EditorLine {
    text_area: TextArea<TokenSequence>,
    pub index: usize,
}

impl EditorLine {
    pub fn new(prompt_width: usize, line_index: usize) -> Self {
        let line_label_width = number_bit_count(line_index) + 1;
        Self {
            text_area: TextArea::new(prompt_width, line_label_width),
            index: line_index,
        }
    }

    pub fn edit(&mut self, key: KeyCode) -> io::Result<()> {
        let text_area = &mut self.text_area;
        match key {
            KeyCode::Backspace => {
                text_area.delete_char()?;
            }

            KeyCode::Left => text_area.move_cursor_horizontal(Direction::Left)?,
            KeyCode::Right => text_area.move_cursor_horizontal(Direction::Right)?,
            KeyCode::Char(ch) => text_area.insert_char(ch)?,
            _ => unreachable!(),
        }
        return Ok(());
    }

    #[inline]
    pub fn rerender(&self) -> io::Result<()> {
        self.text_area.render()
    }

    pub fn set_content(&mut self, new_content: &str) -> io::Result<()> {
        self.text_area.set_content(new_content);
        self.text_area.move_cursor_to_end()?;
        self.rerender()?;
        return Ok(());
    }

    #[inline]
    pub fn content<'a>(&'a self) -> &'a str {
        self.text_area.content()
    }
}
