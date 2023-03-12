use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::exec::attempt::attempt;
use crate::public::number::Number;
use crate::public::ast::ASTNode;

type FileBuf = io::BufReader<File>;
fn read_lines(path: String) -> io::Result<io::Lines<FileBuf>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run_script(
    path: String,
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>,
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
                    script_line,
                    build_in_funcs,
                    variables,
                    goto_statements
                ) {
                    res_number
                } else {
                    Number::NotANumber
                };
            },
            None => {
                println!("{}", last_result);
                break;
            },
        }
    }
}