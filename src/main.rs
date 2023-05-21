mod compiler;
mod computer;
mod exec;
mod public;

use std::collections::VecDeque;
use std::env;

use exec::args;
use public::env::Env;
use public::run_time::scope::Scope;

fn main() {
    let scope = Scope::init();
    let mut args: VecDeque<String> = env::args().collect();

    let self_name = args.pop_front().unwrap();
    let calc_env = Env::init(self_name);

    args::entry(args, calc_env, scope);
}
