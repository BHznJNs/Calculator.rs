mod public;
mod compiler;
mod computer;

use std::collections::HashMap;
use std::io::{self, Write};

use public::ast::ASTNode;
use public::number::Number;
use public::build_in;
use compiler::compile;
use computer::compute;

fn attempt(
    input: String,
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>
) -> Result<Number, ()> {
    let root_node = compile::compile(input)?;
    let result_num = compute::compute(
        root_node,
        build_in_funcs,
        variables,
        goto_statements,
    )?;

    Ok(result_num)
}

fn main() -> ! {
    let build_in_inst = build_in::BuildIn::init();
    let build_in_funcs = build_in::build_in_funcs(&build_in_inst);
    let mut variables = build_in::variables(&build_in_inst);
    let mut goto_statements = HashMap::<String, ASTNode>::new();

    // print program name and version
    println!("Calculator.rs v1.0.0");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(
            input,
            &build_in_funcs,
            &mut variables,
            &mut goto_statements,
        );

        if let Ok(num) = result {
            println!("= {}", num);
        }
    }
}