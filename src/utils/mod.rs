mod cursor;
mod loop_traverser;
mod terminal;

pub mod ascii;
pub mod completer;
pub mod editor;

use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use cursor::Cursor;
use loop_traverser::LoopTraverser;
use terminal::Terminal;

// returns the bit count of number
// e.g. `10` -> 2
//      `1`  -> 1
pub fn number_bit_count(mut num: usize) -> usize {
    if num == 0 {
        return 1;
    }

    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    return count;
}

// this function is used to replace Rust macro `println!`
// since the println! macro can not normally
// make new line in raw_mode.
pub fn print_line<T: Display>(content: T) {
    print!("{}\r\n", content);
    Terminal::flush().expect("IO Error");
}

// output something into file
// this function is used to debug.
#[allow(dead_code)]
pub fn log<T: Display>(content: T) -> io::Result<()> {
    File::create("log.txt")?;
    let mut file = OpenOptions::new().write(true).open("log.txt")?;
    file.write(content.to_string().as_bytes())?;
    file.flush()?;
    return Ok(());
}
