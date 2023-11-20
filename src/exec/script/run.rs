use std::time::Instant;

use crate::{
    exec::{attempt, script::pre_processer::CodeLine},
    public::{env::ENV, run_time::scope::Scope},
    utils::OutputBuffer,
};

use super::{pre_processer, read_file};

#[derive(Default)]
struct MultiLine {
    is_multi_line: bool,
    start_indent: usize,
    code_to_execute: String,
    pub content: String,
}
impl MultiLine {
    pub fn append_line(&mut self, line: &CodeLine) {
        use std::mem::take;

        if self.content.is_empty() {
            self.content.push_str(&line.content);
            self.start_indent = line.indent;
            return;
        }

        if line.indent > self.start_indent {
            // start or in a multi-line block
            self.is_multi_line = true;
            self.content.push_str(&line.content);
            // automatically append semicolon(';')
            if !line.content.ends_with('{') {
                self.content.push(';');
            }
            return;
        }

        // imply that `line.indent <= self.start_indent`
        if !self.is_multi_line {
            // single code line
            self.code_to_execute = take(&mut self.content);
            self.content.push_str(&line.content);
        } else {
            // end a multi-line block
            self.is_multi_line = false;
            self.start_indent = line.indent;
            self.content.push_str(&line.content);
            self.code_to_execute = take(&mut self.content);
        }
    }
    pub fn take(&mut self) -> Option<String> {
        use std::mem::take;
        if self.code_to_execute.is_empty() {
            return None;
        } else {
            return Some(take(&mut self.code_to_execute));
        }
    }
}

// internally read file and run
fn run_with_path(path: &str, scope: &mut Scope) {
    let file_content =
        read_file(path).unwrap_or_else(|_| panic!("code file \"{}\" read error", path));
    self::run(&file_content, scope);
}

// run with lines of codes
fn run(codes: &str, scope: &mut Scope) {
    fn script_run_error(err: String, line_index: usize, code: &str) {
        OutputBuffer::error_append(&(err + "\r\n"), false);
        OutputBuffer::error_append(&format!("Error occured at line {}.\r\n", line_index), false);
        OutputBuffer::error_append(&format!("Code: `{}`.", code), true);
    }

    // --- --- --- --- --- ---

    let mut cached_multiline = MultiLine::default();
    for (index, l) in codes.lines().enumerate() {
        // skip blank line
        if l.trim().is_empty() {
            continue;
        }
        let current_line = pre_processer::process(l);
        let current_index = index + 1;
        cached_multiline.append_line(&current_line);

        let Some(line_to_execute) = cached_multiline.take() else {
            continue;
        };
        // execute code
        if let Err(err) = attempt(&line_to_execute, scope) {
            script_run_error(err, current_index, &current_line.content);
            break;
        }
    }
    // to execute last line of script
    if !cached_multiline.content.is_empty() {
        let last_line_content = cached_multiline.content;
        if let Err(err) = attempt(&last_line_content, scope) {
            let line_count = codes.lines().count();
            script_run_error(err, line_count, &last_line_content);
        }
    }
}

// --- --- --- --- --- ---

// callback function aliases
pub const RUN: fn(&str, &mut Scope) = run;
pub const RUN_PATH: fn(&str, &mut Scope) = run_with_path;

// this function is the external interface to
// call the internal function `run` and `run_with_path`;
// callback here can be `RUN` and `RUN_PATH`.
pub fn run_entry(s: &str, scope: &mut Scope, callback: fn(s: &str, scope: &mut Scope)) {
    if unsafe { ENV.options.timer } {
        let now = Instant::now();

        callback(s, scope);
        let elapsed_time = now.elapsed();
        let elapsed_second = elapsed_time.as_secs_f64();
        OutputBuffer::print_append(&format!("Executed in: {}s.", elapsed_second), true);
    } else {
        callback(s, scope);
    }
}

#[test]
fn muili_line_test() {
    let test_file_content = "\
out 'test content 1'
out 'test content 2'

fn test_function(i) {
    a = 1
    b = 2

    if a == 1 & b == 2 {
        out 'true message1'
        out 'true message2'
    }

    c = 3
    d = 4
    brk i + a + b
}

out test_function(5)
";

    let mut cached_multiline = MultiLine::default();
    for line_content in test_file_content.lines() {
        // skip blank line
        if line_content.trim().is_empty() {
            continue;
        }

        let current_line = pre_processer::process(line_content);
        cached_multiline.append_line(&current_line);
        let Some(line_to_execute) = cached_multiline.take() else {
            continue;
        };
        println!("{line_to_execute}");
    }
    println!("{}", cached_multiline.content);
}