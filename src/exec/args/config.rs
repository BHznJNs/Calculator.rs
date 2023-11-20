use std::{collections::VecDeque, env::current_exe, io};

use crate::{
    exec::{
        args::{data::ARG_CONFIGURABLE, Arg},
        headfile,
        script,
    },
    public::{env::ENV, run_time::scope::Scope, value::Value},
    utils::editor::CodeEditor,
};

pub fn load_config(used_args: Vec<bool>, scope: &mut Scope) -> io::Result<()> {
    const CONFIG_FILE_NAME: &str = "config.calcrs";

    let timer_arg_resolver = |arg_value: Value| {
        let bool_value = arg_value.get_bool();
        unsafe { ENV.options.timer = bool_value };
    };
    let mut headfile_arg_resolver = |arg_value: Value| {
        let Value::Array(arr) = arg_value else {
            return;
        };

        let temp = arr.borrow();
        let headfiles: VecDeque<String> = temp.iter().map(|v| v.to_raw_string()).collect();
        headfile::resolve(&headfiles, scope);
        unsafe { ENV.headfiles = Some(Vec::from(headfiles)) };
    };
    let accent_color_arg_resolver = |arg_value: Value| {
        let color_value = arg_value.to_raw_string();
        CodeEditor::set_accent_color(&color_value);
    };
    let indent_size_arg_resolver = |arg_value: Value| {
        let Ok(indent_size) = arg_value.get_i64() else {
            return;
        };
        unsafe { ENV.options.indent_size = indent_size as usize }
    };

    // --- --- --- --- --- ---

    let base = &current_exe()?;
    let parent = base.parent().unwrap();
    let config_path = parent.join(CONFIG_FILE_NAME);
    let config_path_str = config_path.to_str();

    if !config_path.exists() || config_path_str.is_none() {
        return Ok(());
    }

    let mut config_scope = Scope::new();
    script::RUN_PATH(config_path_str.unwrap(), &mut config_scope);

    for (arg_str, arg_target) in ARG_CONFIGURABLE {
        let Ok(arg_value) = config_scope.read_var(arg_str) else {
            continue;
        };
        if used_args[arg_target as usize] {
            // skip the args that is set in the command line args.
            continue;
        }
        match arg_target {
            Arg::Timer => timer_arg_resolver(arg_value),
            Arg::Headfile => headfile_arg_resolver(arg_value),
            Arg::AccentColor => accent_color_arg_resolver(arg_value),
            Arg::IndentSize => indent_size_arg_resolver(arg_value),
            _ => unreachable!(),
        }
    }
    return Ok(());
}
