mod public;
mod compiler;
mod computer;
mod exec;

use std::env;

use public::run_time::global::Global;
use exec::repl::repl;
use exec::run_script::run_script;

fn main() {
    let global = Global::init();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // REPL mode
            repl(global);
        },
        2 => {
            // script mode
            run_script(args[1].to_owned(), global);
        },
        _ => {
            println!("Too many args.");
        }
    }
}