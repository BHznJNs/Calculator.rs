use std::io::{self, Write};

use super::attempt::attempt;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

fn import_all(
    scope: &mut Scope
) -> Result<(), ()> {
    scope.import("Basic")?;
    scope.import("Math")?;
    scope.import("Array")?;
    Ok(())
}

pub fn repl(mut scope: Scope) -> ! {
    // print program name and version
    println!("Calculator.rs v1.4.1");
    // import stantard libraries
    if import_all(&mut scope).is_err() {
        println!("Standard module import error.");
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(input, &mut scope);

        if let Ok(val_box) = result {
            let val = val_box.as_ref();
            if let Value::Number(Number::Empty(_)) = val {
                continue;
            }
            println!("= {}", val);
        }
    }
}