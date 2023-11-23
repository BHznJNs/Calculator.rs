mod support_keyboard_enhancement;

use std::io;
use std::time::Instant;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use super::attempt;
use crate::public::env::{Env, ENV};
use crate::public::error::{import_error, CalcResult};
use crate::public::run_time::scope::Scope;
use crate::public::std::StdModules;
use crate::public::value::Value;
use crate::utils::editor::{LineEditor, Signal};
use crate::utils::OutputBuffer;

fn import_all(scope: &mut Scope) -> CalcResult<()> {
    for module_name in StdModules::STD_MODULE_IDENTIFIERS {
        scope.import_std(module_name)?;
    }
    return Ok(());
}

#[cfg(windows)]
fn is_ansi_supported_setter() {
    use crossterm::ansi_support::supports_ansi;

    unsafe { ENV.options.support_ansi = supports_ansi() };
}

#[cfg(not(windows))]
fn is_ansi_supported_setter() {
    unsafe { ENV.options.support_ansi = true };
}

pub fn repl(scope: &mut Scope) -> io::Result<()> {
    scope.init_completer();
    // scope.completer = Some(Completer::new());

    // print program name and version
    println!("Calculator.rs v{}", Env::VERSION);
    // set is terminal support ANSI
    is_ansi_supported_setter();
    // import stantard libraries
    if import_all(scope).is_err() {
        let err = import_error("standard module import error");
        OutputBuffer::error_append(&err, true);
    }

    let mut editor = LineEditor::new()?;
    enable_raw_mode()?;

    loop {
        support_keyboard_enhancement::resolve()?;

        let sig = editor.readline(scope)?;
        let line_content = match sig {
            Signal::NewLine(line) => line,
            Signal::Interrupt => break,
            Signal::NonASCII => continue,
        };

        let result;
        if unsafe { ENV.options.timer } {
            let now = Instant::now();
            result = attempt(&line_content, scope);
            let elapsed_time = now.elapsed();
            let elapsed_second = elapsed_time.as_secs_f64();
            let timer_msg = format!("Executed in: {}s.", elapsed_second);
            OutputBuffer::print_append(&timer_msg, true);
        } else {
            result = attempt(&line_content, scope);
        }

        match result {
            Ok(val) => match val {
                Value::Void(_) => continue,
                Value::String(_) => {
                    OutputBuffer::print_append("= ", false);
                    OutputBuffer::print_append(&val.str_format().unwrap(), true);
                }
                _ => {
                    OutputBuffer::print_append("= ", false);
                    OutputBuffer::print_append(&val.to_string(), true);
                }
            }
            Err(err) => OutputBuffer::error_append(&err, true),
        }
    }
    disable_raw_mode()?;
    return Ok(());
}
