use std::io;

use crossterm::{event::KeyCode, style::Stylize};

use crate::utils::{
    editor::code_editor::{direction::Direction, text_area::TextArea},
    Cursor, Terminal,
};

#[derive(Clone)]
pub struct ComponentController {
    pub prompt: &'static str,
    pub button: &'static str,
    pub text_area: TextArea,

    // verticle position to show,
    // if less than zero, equal to
    // Terminal::height() - position - 1
    pub position: isize,

    pub editable: bool,
}

impl ComponentController {
    pub fn open(&mut self) -> io::Result<()> {
        let render_pos = if self.position >= 0 {
            self.position as usize
        } else {
            (Terminal::height() as isize + self.position - 1) as usize
        };

        Cursor::move_to_row(render_pos)?;
        Cursor::move_to_col(0)?;
        print!("{}", self.prompt.bold().black().on_white());

        Cursor::move_to_col(Terminal::width() - self.button.len())?;
        print!("{}", self.button.bold().black().on_white());

        self.text_area.render()?;
        self.text_area.move_cursor_to_end()?;
        return Ok(());
    }

    #[inline]
    pub fn is_editing_key(key: KeyCode) -> bool {
        match key {
            KeyCode::Backspace | KeyCode::Left | KeyCode::Right | KeyCode::Char(_) => true,
            _ => false,
        }
    }

    pub fn edit(&mut self, key: KeyCode) -> io::Result<()> {
        if !self.editable {
            return Ok(());
        }

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
}
