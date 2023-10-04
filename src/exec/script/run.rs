use std::time::Instant;

use crate::{
    exec::attempt,
    public::{env::ENV, run_time::scope::Scope},
};

use super::{pre_processer, read_file};

pub fn run_with_path(path: &str, scope: &mut Scope) {
    let file_content =
        read_file(path).unwrap_or_else(|_| panic!("head file \"{}\" read error", path));
    self::run(&file_content, scope);
}

pub fn run(codes: &str, scope: &mut Scope) {
    let mut cached_multiline = String::new();
    let mut brace_count = 0;

    for (index, l) in codes.lines().enumerate() {
        let mut current_line = pre_processer::process(l);

        // skip blank line
        if current_line.is_empty() {
            continue;
        }

        if current_line.ends_with('{') {
            brace_count += 1;
        }
        if current_line.ends_with('}') {
            brace_count -= 1;
            if brace_count > 0 {
                // nested function or class
                current_line.push(';');
            }
        }

        match brace_count {
            x if x <= 0 => {
                let line_to_exec = if cached_multiline.is_empty() {
                    &current_line
                } else {
                    cached_multiline.push_str(&current_line);
                    &cached_multiline
                };
                // execuse the line
                let line_result = attempt(line_to_exec, scope);
                cached_multiline.clear();

                if line_result.is_err() {
                    println!("Error occured at line {}.", index + 1);
                    // print error code
                    println!("Code: `{}`.", current_line);
                    break;
                }
            }
            x if x > 0 => cached_multiline.push_str(&current_line),
            _ => unreachable!(),
        }
    }
}

// callback here can be `run` and `run_with_path`.
pub fn run_entry(s: &str, scope: &mut Scope, callback: fn(s: &str, scope: &mut Scope)) {
    if unsafe { ENV.options.timer } {
        let now = Instant::now();

        callback(s, scope);
        let elapsed_time = now.elapsed();
        let elapsed_second = elapsed_time.as_secs_f64();
        println!("Executed in: {}s.", elapsed_second);
    } else {
        callback(s, scope);
    }
}
