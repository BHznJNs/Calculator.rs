use std::{
    fmt::Display,
    io::{stdout, Stdout},
};

use crossterm::{cursor, execute};

// this function is used to
// to be compatible with Unix OS
// when is in raw_mode of readline
pub fn print_line<T: Display>(stdout: &mut Stdout, content: T) {
    println!("{}", content);
    execute!(stdout, cursor::MoveToColumn(0)).unwrap();
}

// this function do the same thing with `print_line`,
// but does not receive `stdout`
pub fn print_line__<T: Display>(content: T) {
    println!("{}", content);
    execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
}
