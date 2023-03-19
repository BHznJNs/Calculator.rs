use std::fs::File;
use std::io::{self, BufRead};

use crate::exec::attempt::attempt;
use crate::public::global::Global;

type FileBuf = io::BufReader<File>;
fn read_lines(path: String) -> io::Result<io::Lines<FileBuf>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run_script(
    path: String,
    mut global: Global
) {
    let mut script_lines =
    if let Ok(lines) = read_lines(path) {
        lines
    } else {
        println!("Invalid file.");
        return
    };

    loop {
        match script_lines.next() {
            Some(item) => {
                let mut script_line =
                if let Ok(line) = item {
                    line
                } else {
                    String::new()
                };
                script_line.push('\r');

                let line_result =
                    attempt(script_line, &mut global);
                if line_result.is_err() {
                    break;
                }
            },
            // if is the last line
            None => break,
        }
    }
}