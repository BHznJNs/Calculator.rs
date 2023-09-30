use std::{fmt, io};

use crate::utils::Cursor;

// use to indicate virtual cursor position
// in editing area.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EditorCursorPos {
    pub row: usize,
    pub col: usize,
}

impl EditorCursorPos {
    pub fn short_display(&self) -> String {
        format!("{}: {}", self.row, self.col)
    }

    #[allow(unused_assignments)]
    pub fn parse(value: &str) -> Option<Self> {
        fn str_to_num(s: &str) -> Option<usize> {
            match s.parse::<usize>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        }

        let mut chars = value.chars();
        let mut number_str = String::new();
        let (mut row, mut col) = (1, 1);

        while let Some(ch) = chars.next() {
            if ch.is_ascii_digit() {
                number_str.push(ch);
            } else {
                match ch {
                    ',' | ':' => {
                        row = str_to_num(&number_str)?;
                        number_str.clear();
                    }
                    ' ' => continue,
                    _ => return None,
                }
            }
        }
        col = str_to_num(&number_str)?;
        return Some(EditorCursorPos { row, col });
    }
}

// default display
impl fmt::Display for EditorCursorPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.row, self.col)
    }
}

// --- --- --- --- --- ---

// use to temporarily save and restore cursor.
pub struct TerminalCursorPos {
    pub row: usize,
    pub col: usize,
}

impl TerminalCursorPos {
    #[inline]
    pub fn save_pos(&mut self) -> io::Result<()> {
        (self.row, self.col) = (Cursor::pos_row()?, Cursor::pos_col()?);
        return Ok(());
    }

    #[inline]
    pub fn restore_pos(&self) -> io::Result<()> {
        Cursor::move_to_row(self.row)?;
        Cursor::move_to_col(self.col)?;
        return Ok(());
    }
}
