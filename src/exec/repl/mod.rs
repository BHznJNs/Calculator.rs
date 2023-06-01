mod support_keyboard_enhancement;

use std::io;
use std::time::Instant;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use super::attempt::attempt;
use crate::public::env::Env;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::utils::line_editor::{LineEditor, Signal};

fn import_all(scope: &mut Scope) -> Result<(), ()> {
    scope.import_std("Basic")?;
    scope.import_std("Math")?;
    scope.import_std("String")?;
    scope.import_std("Array")?;
    Ok(())
}

const PROMPT: &'static str = "> ";

pub fn repl(scope: &mut Scope, calc_env: Env) -> io::Result<()> {
    // print program name and version
    println!("Calculator.rs v{}", calc_env.version);
    // import stantard libraries
    if import_all(scope).is_err() {
        println!("Standard module import error.");
    }

    enable_raw_mode()?;

    let mut rl = LineEditor::new(PROMPT);
    loop {
        support_keyboard_enhancement::resolve()?;

        let sig = rl.readline()?;
        let line_content =
        match sig {
            Signal::NewLine(line) => line + "\r\n",
            Signal::Interrupt => break,
            Signal::NonASCII  => todo!(),
        };

        let result: Result<Value, ()>;
        if calc_env.timer {
            let now = Instant::now();
            result = attempt(&line_content, scope);
            let elapsed_time = now.elapsed();
            let elapsed_second = elapsed_time.as_secs_f64();
            println!("Executed in: {}s.", elapsed_second);
        } else {
            result = attempt(&line_content, scope);
        }

        if let Ok(val) = result {
            if let Value::Void(_) = val {
                continue;
            } else if let Value::String(_) = val {
                println!("= {}", val.str_format());
            } else {
                println!("= {}", val);
            }
        }
    }
    disable_raw_mode()
}
