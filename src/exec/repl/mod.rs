mod support_keyboard_enhancement;

use std::io;
use std::time::Instant;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use super::attempt::attempt;
use crate::public::env::{Env, ENV_OPTION};
use crate::public::error::{import_error, syntax_error};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::utils::completer::Completer;
use crate::utils::line_editor::{LineEditor, Signal};
use crate::utils::output::print_line;

const PROMPT: &'static str = "> ";

fn import_all(scope: &mut Scope) -> Result<(), ()> {
    scope.import_std("Basic")?;
    scope.import_std("Math")?;
    scope.import_std("String")?;
    scope.import_std("Array")?;
    scope.import_std("Map")?;
    scope.import_std("FS")?;
    scope.import_std("BitOps")?;
    Ok(())
}

#[cfg(windows)]
fn is_ansi_supported_setter() {
    use crossterm::ansi_support::supports_ansi;
    unsafe { ENV_OPTION.support_ansi = supports_ansi() };
}

#[cfg(not(windows))]
fn is_ansi_supported_setter() {
    unsafe { ENV_OPTION.support_ansi = true };
}

pub fn repl(scope: &mut Scope, calc_env: Env) -> io::Result<()> {
    unsafe { ENV_OPTION.is_repl = true };
    scope.completer = Some(Completer::new());

    // print program name and version
    println!("Calculator.rs v{}", calc_env.version);
    // set is terminal support ANSI
    is_ansi_supported_setter();
    // import stantard libraries
    if import_all(scope).is_err() {
        import_error("standard module import error").unwrap_err();
        panic!()
    }

    enable_raw_mode()?;

    let mut rl = LineEditor::new(PROMPT);
    let mut stdout = io::stdout();
    loop {
        support_keyboard_enhancement::resolve()?;

        let sig = rl.readline(scope)?;
        let line_content = match sig {
            Signal::NewLine(line) => line,
            Signal::Interrupt => break,
            Signal::NonASCII => {
                syntax_error("non-ASCII character").unwrap_err();
                continue;
            }
        };

        let result: Result<Value, ()>;
        if unsafe { ENV_OPTION.timer } {
            let now = Instant::now();
            result = attempt(&line_content, scope);
            let elapsed_time = now.elapsed();
            let elapsed_second = elapsed_time.as_secs_f64();
            print_line(&mut stdout, format!("Executed in: {}s.", elapsed_second));
        } else {
            result = attempt(&line_content, scope);
        }

        if let Ok(val) = result {
            if let Value::Void(_) = val {
                continue;
            } else if let Value::String(_) = val {
                print!("= ");
                print_line(&mut stdout, val.str_format().unwrap());
            } else {
                print!("= ");
                print_line(&mut stdout, val);
            }
        }
    }
    disable_raw_mode()
}
