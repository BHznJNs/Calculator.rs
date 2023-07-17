mod cursor;

use std::io::{self, Stdout, Write};

use crossterm::{
    event::{self, Event, KeyEvent, KeyEventKind},
    terminal::size,
};

pub use cursor::Cursor;

pub struct Terminal {
    pub stdout: Stdout,
    pub cursor: Cursor,
}

const BACKSPACE: &'static str = "\x1B[K";

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdout: io::stdout(),
            cursor: Cursor::new(),
        }
    }

    pub fn get_key(&self) -> Option<KeyEvent> {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                return Some(key);
            }
        }
        None
    }

    pub fn width(&self) -> usize {
        size().unwrap().0 as usize
    }

    pub fn cursor_col(&self) -> io::Result<usize> {
        Ok(self.cursor.position()?.0)
    }

    // --- --- --- --- --- ---

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn clear_after_cursor(&mut self) {
        print!("{}", BACKSPACE);
    }
}
