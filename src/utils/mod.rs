mod cursor;
mod terminal;

pub mod ascii;
pub mod completer;
pub mod line_editor;

use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use terminal::Terminal;

// this function is used to replace Rust macro `println!`
// since the println! macro can not normally
// make new line in raw_mode.
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
