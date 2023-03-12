mod public;
mod compiler;
mod computer;
mod exec;

use std::env;
use std::collections::HashMap;

use public::ast::ASTNode;
use public::build_in;
use exec::repl::repl;
use exec::run_script::run_script;

fn main() {
    let build_in_inst = build_in::BuildIn::init();
    let build_in_funcs = build_in::build_in_funcs(&build_in_inst);
    let mut variables = build_in::variables(&build_in_inst);
    let mut goto_statements = HashMap::<String, ASTNode>::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            // REPL mode
            repl(
                &build_in_funcs,
                &mut variables,
                &mut goto_statements,
            );
        },
        2 => {
            // script mode
            run_script(
                args[1].to_owned(),
                &build_in_funcs,
                &mut variables,
                &mut goto_statements,
            );
        },
        _ => {
            println!("Too many args.");
        }
    }
}