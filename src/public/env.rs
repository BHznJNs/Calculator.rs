use crate::exec::args::commands;

pub struct Env {
    pub self_name: &'static str,

    pub script_path: Option<&'static str>,
    pub headfiles: &'static[String],

    pub options: EnvOption,
}

pub struct EnvOption {
    pub timer: bool,
    pub use_repl: bool,
    pub use_editor: bool,
    pub support_ansi: bool,
    pub indent_size: usize, // the count of space for indent
}

pub static mut ENV: Env = Env {
    self_name: "",
    script_path: None,
    headfiles: &[],

    options: EnvOption {
        timer: false,
        use_repl: false,
        use_editor: false,
        support_ansi: false,
        indent_size: 2,
    },
};

impl Env {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn version_output() {
        println!("Calculator.rs version {}", Self::VERSION);
    }

    pub fn help_output() {
        println!("Usage: calculator [SCRIPT_PATH] [OPTIONS]\n");

        println!("Options:");
        for i in 0..commands::COMMAND_COUNT {
            println!("{}, {}", commands::COMMANDS[i][0], commands::COMMANDS[i][1]);
            println!("  {}", commands::COMMAND_DESCRIPTIONS[i]);
        }
    }
}
