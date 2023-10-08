use std::io;

use crossterm::{cursor, execute};

pub struct Cursor;

impl Cursor {
    // return current cursor col
    fn position() -> io::Result<(usize, usize)> {
        let (col, row) = cursor::position()?;
        Ok((col as usize, row as usize))
    }

    pub fn pos_col() -> io::Result<usize> {
        Ok(Self::position()?.0)
    }
    pub fn pos_row() -> io::Result<usize> {
        Ok(Self::position()?.1)
    }

    pub fn move_to_col(target_col: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveToColumn(target_col as u16))
    }
    pub fn move_to_row(target_row: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveToRow(target_row as u16))
    }

    pub fn up(cell: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveUp(cell as u16))
    }
    pub fn down(cell: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveDown(cell as u16))
    }
    pub fn left(cell: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveLeft(cell as u16))
    }
    pub fn right(cell: usize) -> io::Result<()> {
        execute!(io::stdout(), cursor::MoveRight(cell as u16))
    }

    pub fn save_pos() -> io::Result<()> {
        execute!(io::stdout(), cursor::SavePosition)
    }
    pub fn restore_pos() -> io::Result<()> {
        execute!(io::stdout(), cursor::RestorePosition)
    }
}
