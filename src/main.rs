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
}