mod config;
mod data;
mod resolver;

pub use config::load_config;
pub use data::{ARGS, ARG_COUNT, ARG_DESCRIPTIONS};
pub use resolver::args_resolve;

use std::collections::HashMap;

#[repr(usize)]
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Arg {
    Timer,
    Headfile,
    Editor,
    AccentColor,
    IndentSize,

    Version,
    Help,
}

impl Arg {
    pub(super) fn map() -> HashMap<&'static str, Arg> {
        use std::mem::transmute;
        let into_arg = |num: usize| unsafe { transmute::<usize, Arg>(num) };

        let mut result_map = HashMap::new();
        for (index, command_str_pair) in ARGS.iter().enumerate() {
            let target_arg = into_arg(index);
            result_map.insert(command_str_pair[0], target_arg);
            result_map.insert(command_str_pair[1], target_arg);
        }
        return result_map;
    }
}
