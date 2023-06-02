pub mod commands;
mod help_msg;

use std::{collections::VecDeque, io, process};

use crate::public::{env::{Env, ENV_OPTION}, run_time::scope::Scope};

use super::{headfile, repl::repl, script};
use commands::CommandArg;

enum Mode {
    REPL,
    Script,
}

fn args_resolve(
    mode: Mode,
    mut args: VecDeque<String>,
    mut calc_env: Env,
    mut scope: Scope,
) -> io::Result<()> {
    let command_map = CommandArg::map();

    loop {
        // ensure index is not out of range
        if args.len() == 0 {
            break;
        }

        let current_arg = args.pop_front().unwrap();
        if let Some(command) = command_map.get::<str>(&current_arg) {
            match command {
                CommandArg::Timer =>
                    unsafe { ENV_OPTION.timer = true },
                CommandArg::Help => {
                    help_msg::output();
                    process::exit(0);
                }
                CommandArg::Version => {
                    calc_env.version_output();
                    process::exit(0);
                }
                CommandArg::Headfile => {
                    // remaining args as headfile
                    calc_env.headfiles = args.clone();
                    headfile::resolve(args, &mut scope);
                    break;
                }
            }
        } else {
            println!("Invalid command: {}.", current_arg);
            process::exit(0);
        }
    }

    match mode {
        Mode::REPL => repl(&mut scope, calc_env)?,
        Mode::Script => script::env_resolve(calc_env, &mut scope),
    }
    Ok(())
}

pub fn entry(mut args: VecDeque<String>, mut calc_env: Env, mut scope: Scope) -> io::Result<()> {
    if args.len() == 0 {
        // if no argument, enter REPL directly.
        repl(&mut scope, calc_env)?;
        return Ok(());
    }

    // consider execute mode
    let mode = if args[0].starts_with('-') || args[0].starts_with("--") {
        // REPL mode
        Mode::REPL
    } else {
        // first arg is script path
        let script_path = args.pop_front().unwrap();
        calc_env.script_path = Some(script_path);
        Mode::Script
    };

    args_resolve(mode, args, calc_env, scope)
}
