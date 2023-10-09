pub mod commands;

use std::{collections::VecDeque, io};

use crate::{
    public::{
        env::{Env, ENV},
        error::{internal_error, InternalComponent},
        run_time::scope::Scope,
    },
    utils::editor::CodeEditor,
    ProgramMode,
};

use super::headfile;
use commands::CommandArg;

#[inline]
fn is_command_str(str: &str) -> bool {
    str.starts_with('-') || str.starts_with("--")
}

pub fn args_resolve(mut args: VecDeque<String>, scope: &mut Scope) -> io::Result<ProgramMode> {
    // argument resolvers
    let timer_arg_resolver = || unsafe { ENV.options.timer = true };
    let headfile_arg_resolver = |args: VecDeque<String>, scope: &mut Scope| {
        // regard remain arguments as headfile paths
        let headfile_paths = args;
        headfile::resolve(&headfile_paths, scope);
        unsafe { ENV.headfiles = Some(Vec::from(headfile_paths)) };
    };
    let editor_arg_resolver = |mode: &mut ProgramMode| {
        if *mode != ProgramMode::ToBeExited {
            *mode = ProgramMode::Editor;
        }
    };
    let accent_color_arg_resolver = |args: &mut VecDeque<String>| {
        if let Some(color_value) = args.pop_front() {
            CodeEditor::set_accent_color(&color_value);
        }
    };
    let indent_size_arg_resolver = |args: &mut VecDeque<String>| {
        if let Some(indent_size_str) = args.pop_front() {
            if let Ok(indent_size) = indent_size_str.parse::<usize>() {
                unsafe { ENV.options.indent_size = indent_size };
            }
        }
    };

    // --- --- --- --- --- ---

    let mut mode;
    let command_map = CommandArg::map();

    if args.get(0).is_some_and(|arg| is_command_str(arg)) {
        mode = ProgramMode::REPL;
        unsafe { ENV.options.use_repl = true };
    } else {
        // first argument as file path
        mode = ProgramMode::Script;
        let script_path = args.pop_front().unwrap();
        let static_path = Box::leak(Box::new(script_path));
        unsafe { ENV.script_path = Some(static_path) };
    }

    while let Some(arg) = args.pop_front() {
        if let Some(command) = command_map.get::<str>(&arg) {
            match command {
                CommandArg::Help | CommandArg::Version => {
                    match command {
                        CommandArg::Help => Env::help_output(),
                        CommandArg::Version => Env::version_output(),
                        _ => unreachable!(),
                    }
                    mode = ProgramMode::ToBeExited;
                }
                CommandArg::Timer => timer_arg_resolver(),
                CommandArg::Headfile => {
                    headfile_arg_resolver(args, scope);
                    break;
                }
                CommandArg::Editor => editor_arg_resolver(&mut mode),
                CommandArg::AccentColor => accent_color_arg_resolver(&mut args),
                CommandArg::IndentSize => indent_size_arg_resolver(&mut args),
            }
        } else {
            let msg = format!("Invalid argument: {}", arg);
            internal_error(InternalComponent::InternalFn, &msg).unwrap_err();
            mode = ProgramMode::ToBeExited;
            break;
        }
    }
    return Ok(mode);
}
