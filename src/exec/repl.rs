use std::io::{self, Write};

use super::attempt::attempt;
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

pub fn repl(mut global: Global) -> ! {
    // print program name and version
    println!("Calculator.rs v1.3.1");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(input, &mut global);

        if let Ok(val) = result {
            if let Value::Number(Number::Empty(_)) = val {
                continue;
            }
            println!("= {}", val);
        }
    }
}