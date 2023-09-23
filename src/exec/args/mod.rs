pub mod commands;

use std::{collections::VecDeque, io};

use crate::{
    public::{
        env::{Env, ENV},
        error::{internal_error, InternalComponent},
        run_time::scope::Scope,
    },
    ProgramMode,
};

use super::headfile;
use commands::CommandArg;

pub fn args_resolve(mut args: VecDeque<String>, scope: &mut Scope) -> io::Result<ProgramMode> {
    if args.len() == 0 {
        return Ok(ProgramMode::REPL);
    }

    let mut mode;
    let command_map = CommandArg::map();
    if args[0].starts_with('-') || args[0].starts_with("--") {
        mode = ProgramMode::REPL;
        unsafe { ENV.options.use_repl = true };
    } else {
        // first argument as file path
        mode = ProgramMode::Script;
        let script_path = args.pop_front().unwrap();
        let static_path = Box::leak(Box::new(script_path));
        unsafe { ENV.script_path = Some(static_path) };
    }

    for arg in args.iter() {
        if let Some(command) = command_map.get::<str>(arg) {
            match command {
                CommandArg::Help | CommandArg::Version => {
                    match command {
                        CommandArg::Help => Env::help_output(),
                        CommandArg::Version => Env::version_output(),
                        _ => unreachable!(),
                    }
                    mode = ProgramMode::ToBeExited;
                }
                CommandArg::Timer => unsafe { ENV.options.timer = true },
                CommandArg::Headfile => {
                    // regard remain arguments as headfile path.
                    let headfile_paths = args;
                    headfile::resolve(&headfile_paths, scope);

                    let temp = Vec::from(headfile_paths);
                    unsafe { ENV.headfiles = Box::leak(Box::new(temp)) };
                    break;
                }
                CommandArg::Editor => {
                    if mode != ProgramMode::ToBeExited {
                        mode = ProgramMode::Editor;
                    }
                }
            }
        } else {
            let msg = format!("Invalid argument: {}.", arg);
            internal_error(InternalComponent::InternalFn, &msg).unwrap_err();
            mode = ProgramMode::ToBeExited;
        }
    }
    return Ok(mode);
}
