mod terminal;
mod cursor;

pub mod ascii;
pub mod completer;
pub mod line_editor;


use std::{
    io::{
        self,
        Write,
    },
    fmt::Display,
    fs::{
        File,
        OpenOptions,
    },
};

use terminal::Terminal;

pub fn print_line<T: Display>(content: T) {
    print!("{}\r\n", content);
    Terminal::flush().expect("IO Error");
}

// output something into file
// this function is used to debug.
pub fn log(content: &str) -> io::Result<()> {
    File::create("log.txt")?;
    let mut file = OpenOptions::new().write(true).open("log.txt")?;
    file.write(content.as_bytes())?;
    file.flush()?;
    return Ok(());
}
