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

        run(&script_path, scope);
        let elapsed_time = now.elapsed();
        let elapsed_second = elapsed_time.as_secs_f64();
        println!("在 {}s 内执行完成。", elapsed_second);
    } else {
        run(&script_path, scope);
    }
}

pub fn run(path: &str, scope: &mut Scope) {
    let Ok(mut script_lines) = readlines::resolve(path) else {
        println!("Invalid script file.");
        return;
    };

    let mut cached_multiline = String::new();
    let mut line_count = 0;
    let mut brace_count = 0;

    while let Some(Ok(current_line)) = script_lines.next() {
        let mut current_line= pre_processer::process(current_line);
        line_count += 1;

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
                    cached_multiline.extend(current_line.chars());
                    &cached_multiline
                };
                // execuse the line
                let line_result = attempt(line_to_exec, scope);
                cached_multiline.clear();

                if line_result.is_err() {
                    println!("错误发生在第 {} 行。", line_count);
                    // print error code
                    println!("错误代码：“{}”。", current_line);
                    break;
                }
            }
            x if x > 0 => cached_multiline.extend(current_line.chars()),
            _ => unreachable!()
        }
    }
}
