mod public;
mod compiler;
mod computer;
mod exec;

use std::collections::VecDeque;
use std::env;

use public::env::Env;
use public::run_time::scope::Scope;
use exec::args;

fn main() {
    let scope = Scope::init();
    let mut args: VecDeque<String> = env::args().collect();

    let self_name = args.pop_front().unwrap();
    let calc_env = Env::init(self_name);

    args::entry(args, calc_env, scope);

    // match args.len() {
    //     1 => {
    //         // REPL mode
    //         repl(&mut scope);
    //     },
    //     2 => {
    //         // script mode
    //         run_script(args[1].to_owned(), &mut scope);
    //     },
    //     _ => {
    //         println!("Too many args.");
    //     }
    // }
}