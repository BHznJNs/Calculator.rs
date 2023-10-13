pub struct Env {
    pub self_name: &'static str,

    pub script_path: Option<&'static str>,
    pub headfiles: Option<Vec<String>>,

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
    headfiles: None,

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
        use crate::exec::args::{ARGS, ARG_COUNT, ARG_DESCRIPTIONS};

        println!("Usage: calculator [SCRIPT_PATH] [OPTIONS]\n");

        println!("Options:");
        for i in 0..ARG_COUNT {
            println!("{}, {}", ARGS[i][0], ARGS[i][1]);
            println!("  {}", ARG_DESCRIPTIONS[i]);
        }
    }
}
