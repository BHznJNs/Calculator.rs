use std::collections::VecDeque;

pub struct EnvOption {
    pub timer: bool,
    pub support_ansi: bool,
}

pub static mut ENV_OPTION: EnvOption = EnvOption {
    timer: false,
    support_ansi: false,
};

// --- --- --- --- --- ---

pub struct Env {
    pub self_name: String,
    pub version: &'static str,

    pub script_path: Option<String>,
    pub headfiles: VecDeque<String>,
}

impl Env {
    pub fn init(self_name: String) -> Env {
        Env {
            self_name,
            version: env!("CARGO_PKG_VERSION"),

            script_path: None,
            headfiles: VecDeque::<String>::new(),
        }
    }

    pub fn version_output(&self) {
        println!("Calculator.rs version {}", self.version);
    }
}
