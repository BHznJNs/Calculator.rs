use std::collections::HashMap;
use std::io::{self, Write};

use super::attempt::attempt;
use crate::public::number::Number;
use crate::public::ast::ASTNode;

pub fn repl(
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>,
) -> ! {
    // print program name and version
    println!("Calculator.rs v1.2.1");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(
            input,
            build_in_funcs,
            variables,
            goto_statements,
        );

        if let Ok(num) = result {
            println!("= {}", num);
        }
    }
}