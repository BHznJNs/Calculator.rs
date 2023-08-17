use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyEvent, KeyEventKind},
    terminal::size,
};

pub struct Terminal;

impl Terminal {
    const BACKSPACE: &'static str = "\x1B[K";

    pub fn width() -> usize {
        size().unwrap().0 as usize
    }

    pub fn height() -> usize {
        size().unwrap().1 as usize
    }

    pub fn flush() -> io::Result<()> {
        io::stdout().flush()
    }

    pub fn clear_after_cursor() -> io::Result<()> {
        print!("{}", Self::BACKSPACE);
        Self::flush()
    }

    pub fn get_key() -> Option<KeyEvent> {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                return Some(key);
            }
        }
        None
    }
}
