use std::collections::HashMap;

#[repr(usize)]
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum CommandArg {
    Timer,
    Headfile,
    Editor,
    AccentColor,
    IndentSize,

    Version,
    Help,
}

pub const COMMAND_COUNT: usize = 7;
pub const COMMANDS: [[&str; 2]; COMMAND_COUNT] = [
    ["-t", "--timer"],
    ["-hf", "--headfile"],
    ["-e", "--editor"],
    ["-a", "--accent-color"],
    ["-i", "--indent-size"],
    ["-v", "--version"],
    ["-h", "--help"],
];
pub const COMMAND_DESCRIPTIONS: [&str; COMMAND_COUNT] = [
    "print extra execute duration message code execution.",
    "directly import variables in head files, must with script paths following.",
    "open build-in code editor",
    "editor accent color, options: [red, blue, dark_red, dark_blue, dark_grey, dark_cyan, dark_yellow, dark_magenta]",
    "editor indent size, default: 2",

    "print current executable file version and exit.",
    "print this help message.",
];

impl CommandArg {
    pub(super) fn map() -> HashMap<&'static str, CommandArg> {
        use std::mem::transmute;
        let into_arg = |num: usize| unsafe { transmute::<usize, CommandArg>(num) };

        let mut result_map = HashMap::new();
        for (index, command_str_pair) in COMMANDS.iter().enumerate() {
            let target_arg = into_arg(index);
            result_map.insert(command_str_pair[0], target_arg.clone());
            result_map.insert(command_str_pair[1], target_arg);
        }
        return result_map;
    }
}
