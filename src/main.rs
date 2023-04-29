mod public;
mod compiler;
mod computer;
mod exec;

use std::env;

use public::run_time::build_in;
use public::run_time::scope::Scope;
use exec::repl::repl;
use exec::script::run::run_script;

fn main() {
    let mut scope = Scope::init();
    // insert global variables
    scope.global.variables = build_in::variables();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // REPL mode
            repl(&mut scope);
        },
        2 => {
            // script mode
            run_script(args[1].to_owned(), &mut scope);
        },
        _ => {
            println!("Too many args.");
        }
    }
}