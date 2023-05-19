use std::collections::VecDeque;

pub struct Env {
    pub self_name: String,
    pub version: &'static str,

    pub script_path: Option<String>,
    pub headfiles: VecDeque<String>,

    pub timer: bool,
}

impl Env {
    pub fn init(self_name: String) -> Env {
        Env {
            self_name,
            version: env!("CARGO_PKG_VERSION"),

            script_path: None,
            headfiles: VecDeque::<String>::new(),

            timer: false,
        }
    }

    pub fn version_output(&self) {
        println!("Calculator.rs version {}", self.version);
    }
}