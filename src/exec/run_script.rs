use std::fs::File;
use std::io::{self, BufRead};

use crate::exec::attempt::attempt;
use crate::public::global::Global;
use crate::public::number::Number;

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

    // cache last compute result,
    // when the loop ends,
    // print the final result.
    let mut last_result = Number::NotANumber;
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

                last_result =
                if let Ok(res_number) = attempt(
                    script_line, &mut global
                ) {
                    // if line execuse successfully:

                    if res_number != Number::Empty {
                        res_number
                    } else {
                        // if script line is a comment line
                        last_result
                    }
                } else {
                    break;
                };
            },
            None => {
                println!("{}", last_result);
                break;
            },
        }
    }
}