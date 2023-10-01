use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum CommandArg {
    Version,
    Help,
    Timer,
    Headfile,
    Editor,
}

pub const COMMAND_COUNT: usize = 5;
pub const COMMANDS: [[&str; 2]; COMMAND_COUNT] = [
    ["-v", "--version"],
    ["-h", "--help"],
    ["-t", "--timer"],
    ["-hf", "--headfile"],
    ["-e", "--editor"],
];
pub const COMMAND_DESCRIPTIONS: [&str; COMMAND_COUNT] = [
    "print current executable file version and exit.",
    "print this help message.",
    "print extra execute duration message code execution.",
    "directly import variables in head files, must with script paths following.",
    "open build-in code editor",
];

impl CommandArg {
    pub fn map() -> HashMap<&'static str, CommandArg> {
        HashMap::from([
            (COMMANDS[0][0], CommandArg::Version),
            (COMMANDS[0][1], CommandArg::Version),
            (COMMANDS[1][0], CommandArg::Help),
            (COMMANDS[1][1], CommandArg::Help),
            (COMMANDS[2][0], CommandArg::Timer),
            (COMMANDS[2][1], CommandArg::Timer),
            (COMMANDS[3][0], CommandArg::Headfile),
            (COMMANDS[3][1], CommandArg::Headfile),
            (COMMANDS[4][0], CommandArg::Editor),
            (COMMANDS[4][1], CommandArg::Editor),
        ])
    }
}
