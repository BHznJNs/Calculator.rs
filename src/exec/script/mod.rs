mod pre_processer;
pub mod readlines;

use std::time::Instant;

use crate::public::env::{Env, ENV_OPTION};
use crate::public::run_time::scope::Scope;

use super::attempt::attempt;

pub fn env_resolve(calc_env: Env, scope: &mut Scope) {
    let script_path = calc_env.script_path.unwrap();

    if unsafe { ENV_OPTION.timer } {
        let now = Instant::now();

        if let Ok(_) = run(&script_path, scope) {
            let elapsed_time = now.elapsed();
            let elapsed_second = elapsed_time.as_secs_f64();
            println!("Executed in: {}s.", elapsed_second);
        }
    } else {
        let _ = run(&script_path, scope);
    }
}

pub fn run(path: &str, scope: &mut Scope) -> Result<(), ()> {
    let Ok(mut script_lines) = readlines::resolve(path) else {
        println!("Invalid script file.");
        return Err(())
    };

    let mut cached_multiline = String::new();
    let mut line_count = 0;
    loop {
        match script_lines.next() {
            Some(item) => {
                line_count += 1;

                let mut script_line = if let Ok(line) = item {
                    pre_processer::process(line)
                } else {
                    String::new()
                };

                // skip blank line
                if script_line.len() == 0 {
                    continue;
                }

                // multi-line symbol: `:`
                if script_line.ends_with(":") {
                    script_line.pop();
                    cached_multiline += &script_line;
                } else {
                    // out of multi-line statement
                    // or last line of multi-line statement
                    let current_line: &String;

                    if cached_multiline.is_empty() {
                        // out of multi-line statement
                        current_line = &script_line;
                    } else {
                        // the last line of multi-line statement
                        // or the blank line || line comment
                        if script_line.len() == 0 {
                            // skip the blank line and line comment
                            continue;
                        }
                        cached_multiline += &script_line;
                        current_line = &cached_multiline;
                    }

                    // execuse the line
                    let line_result = attempt(current_line, scope);

                    if line_result.is_err() {
                        println!("Error occured at line {}.", line_count);
                        // print error code
                        println!("Code: `{}`.", current_line);
                        break Err(());
                    }

                    if !cached_multiline.is_empty() {
                        cached_multiline.clear();
                    }
                }
            }
            // if is the last line
            None => break Ok(()),
        }
    }
}
