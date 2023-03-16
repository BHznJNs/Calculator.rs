use std::io::{self, Write};

use super::attempt::attempt;
use crate::public::global::Global;
use crate::public::number::Number;

pub fn repl(mut global: Global) -> ! {
    // print program name and version
    println!("Calculator.rs v1.2.2");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(input, &mut global);

        if let Ok(num) = result {
            if num == Number::Empty {
                continue;
            }
            println!("= {}", num);
        }
    }
}