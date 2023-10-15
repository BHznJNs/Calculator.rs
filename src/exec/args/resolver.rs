use std::{collections::VecDeque, io};

use crate::{
    exec::headfile,
    public::{
        env::{Env, ENV},
        error::{internal_error, InternalComponent},
        run_time::scope::Scope,
    },
    utils::{editor::CodeEditor, OutputBuffer},
    ProgramMode,
};

use super::Arg;

pub fn args_resolve(
    mut args: VecDeque<String>,
    used_args: &mut [bool],
    scope: &mut Scope,
) -> io::Result<ProgramMode> {
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

    if args.is_empty() {
        return Ok(ProgramMode::REPL);
    }

    let mut mode;
    let command_map = Arg::map();

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

    while let Some(arg_str) = args.pop_front() {
        if let Some(arg) = command_map.get::<str>(&arg_str) {
            used_args[*arg as usize] = true;

            match arg {
                Arg::Help | Arg::Version => {
                    match arg {
                        Arg::Help => Env::help_output(),
                        Arg::Version => Env::version_output(),
                        _ => unreachable!(),
                    }
                    mode = ProgramMode::ToBeExited;
                }
                Arg::Timer => timer_arg_resolver(),
                Arg::Headfile => {
                    headfile_arg_resolver(args, scope);
                    break;
                }
                Arg::Editor => editor_arg_resolver(&mut mode),
                Arg::AccentColor => accent_color_arg_resolver(&mut args),
                Arg::IndentSize => indent_size_arg_resolver(&mut args),
            }
        } else {
            let msg = format!("Invalid argument: {}", arg_str);
            let err = internal_error(InternalComponent::InternalFn, &msg);
            OutputBuffer::error_append(&err, true);

            mode = ProgramMode::ToBeExited;
            break;
        }
    }
    return Ok(mode);
}
