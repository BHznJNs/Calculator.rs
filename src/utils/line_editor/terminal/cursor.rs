use std::io::{self, Stdout};

use crossterm::{cursor, execute};

pub struct Cursor {
    stdout: Stdout,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            stdout: io::stdout(),
        }
    }

    // return current cursor col
    pub fn position(&self) -> io::Result<(usize, usize)> {
        let (col, row) = cursor::position()?;
        Ok((col as usize, row as usize))
    }

    pub fn move_to_col(&mut self, target_col: usize) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveToColumn(target_col as u16))
    }
    pub fn move_to_row(&mut self, target_row: usize) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveToRow(target_row as u16))
    }

    pub fn left(&mut self, cell: usize) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveLeft(cell as u16))
    }
    pub fn right(&mut self, cell: usize) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveRight(cell as u16))
    }

    pub fn save_pos(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::SavePosition)
    }
    pub fn restore_pos(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::RestorePosition)
    }

    pub fn hide(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)
    }
    pub fn show(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)
    }
}
