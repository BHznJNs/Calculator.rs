use std::io::{self, Write};
use std::time::Instant;

use super::attempt::attempt;
use crate::public::env::Env;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

fn import_all(
    scope: &mut Scope
) -> Result<(), ()> {
    scope.import_std("Basic" )?;
    scope.import_std("Math"  )?;
    scope.import_std("String")?;
    scope.import_std("Array" )?;
    Ok(())
}

pub fn repl(scope: &mut Scope, calc_env: Env) -> ! {
    // print program name and version
    println!("Calculator.rs {}", calc_env.version);
    // import stantard libraries
    if import_all(scope).is_err() {
        println!("Standard module import error.");
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result: Result<Value, ()>;
        if calc_env.timer {
            let now = Instant::now();
            result = attempt(&input, scope);
            let elapsed_time = now.elapsed();
            let elapsed_second = elapsed_time.as_secs_f64();
            println!("Executed in: {}s.", elapsed_second);
        } else {
            result = attempt(&input, scope);
        }

        if let Ok(val) = result {
            if let Value::Void(_) = val {
                continue;
            } else
            if let Value::String(_) = val {
                println!("= \"{}\"", val);
            } else {
                println!("= {}", val);
            }
        }
    }
}