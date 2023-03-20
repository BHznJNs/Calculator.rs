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

    let mut cached_multiline = String::new();
    let mut line_count = 0;
    loop {
        match script_lines.next() {
            Some(item) => {
                line_count += 1;

                let mut script_line =
                if let Ok(line) = item {
                    line
                } else {
                    String::new()
                };

                // multi-line symbol: `:`
                if script_line.ends_with(":") {
                    script_line.pop();
                    cached_multiline += &script_line;
                } else {
                    // out of multi-line statement
                    // or last line of multi-line statement
                    let current_line: String;

                    if cached_multiline.is_empty() {
                        // out of multi-line statement
                        script_line.push('\r');
                        current_line = script_line;
                    } else {
                        // the last line of multi-line statement
                        cached_multiline += &script_line;
                        current_line = cached_multiline.clone();
                        cached_multiline.clear();
                    }

                    // execuse the line
                    let line_result =
                        attempt(current_line, &mut global);
                    if line_result.is_err() {
                        println!("Error occured at line {}", line_count);
                        break;
                    }
                }
            },
            // if is the last line
            None => break,
        }
    }
}