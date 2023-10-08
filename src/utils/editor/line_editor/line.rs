use std::io;

use crossterm::event::KeyCode;

use crate::{utils::{
    editor::{direction::Direction, text_area::TextArea, tokenizer::TokenSequence},
    number_bit_count,
}, public::run_time::scope::Scope};

use super::{candidate::Candidate, analyzer::analyze};

pub struct EditorLine {
    text_area: TextArea<TokenSequence>,
    candidates: Candidate,
    pub index: usize,
}

impl EditorLine {
    pub fn new(prompt_width: usize, line_index: usize) -> Self {
        let line_label_width = number_bit_count(line_index) + 1;
        Self {
            text_area: TextArea::new(prompt_width, line_label_width),
            candidates: Candidate::new(),
            index: line_index,
        }
    }

    pub fn edit(&mut self, key: KeyCode, scope: &Scope) -> io::Result<()> {
        let text_area = &mut self.text_area;
        match key {
            KeyCode::Backspace => {
                text_area.delete_char(true)?;
            }

            KeyCode::Left => text_area.move_cursor_horizontal(Direction::Left, true)?,
            KeyCode::Right => text_area.move_cursor_horizontal(Direction::Right, true)?,
            KeyCode::Char(ch) => {
                text_area.insert_char(ch, true)?;
                // if self.text_area.state_right()?.is_at_content_end {
                //     let Some(tokens) = self.text_area.tokens() else {
                //         return Ok(());
                //     };
                //     let Ok(candidates) = analyze(tokens, scope) else {
                //         return Ok(());
                //     };
                //     self.candidates.set(candidates);
                // }
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
