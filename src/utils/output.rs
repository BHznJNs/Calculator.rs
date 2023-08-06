use std::{
    fmt::Display,
    io::{stdout, Stdout},
};

use crossterm::{cursor, execute};

// these function is used for being compatible in Unix-like OS
// when is in raw_mode of readline


// pub fn print_line<T: Display>(content: T) {
//     print!("{}\r\n", content);
// }


// When there is `stdout` handler in the context of the caller,
// use `print_line`; otherwise, use `print_line__`
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
