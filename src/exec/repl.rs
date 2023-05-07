use std::io::{self, Write};

use super::attempt::attempt;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
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

pub fn repl(scope: &mut Scope) -> ! {
    // print program name and version
    println!("Calculator.rs v1.7.5");
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

        let result =
            attempt(&input, scope);
        
        if let Ok(val) = result {
            if val == Value::Number(Number::Empty) {
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