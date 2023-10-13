#![allow(clippy::needless_return)]

mod compiler;
mod computer;
mod exec;
mod public;
mod utils;

use std::collections::VecDeque;
use std::{env, io};

use exec::args::{args_resolve, load_config, ARG_COUNT};
use exec::repl;
use exec::script::{run_entry, run_with_path};
use public::env::ENV;
use public::run_time::scope::Scope;
use utils::editor::CodeEditor;

#[derive(PartialEq)]
pub enum ProgramMode {
    REPL,
    Script,
    Editor,

    ToBeExited,
}

fn main() -> io::Result<()> {
    let mut args: VecDeque<String> = env::args().collect();
    let mut scope = Scope::new();

    let self_name = args.pop_front().unwrap();
    unsafe { ENV.self_name = Box::leak(Box::new(self_name)) };

    // records the command line args that is inputed
    let mut used_args: Vec<bool> = vec![false; ARG_COUNT];

    let mode = args_resolve(args, &mut used_args, &mut scope)?;
    load_config(used_args, &mut scope)?;

    match mode {
        ProgramMode::REPL => repl::repl(&mut scope)?,
        ProgramMode::ToBeExited => {}

        ProgramMode::Script => {
            let script_path = unsafe { ENV.script_path.unwrap() };
            run_entry(script_path, &mut scope, run_with_path);
        }
        ProgramMode::Editor => {
            let mut editor = CodeEditor::new();
            if let Some(p) = unsafe { ENV.script_path } {
                editor.read_file(p)?;
            }
            editor.init()?;
            editor.cycle(&mut scope)?;
        }
    }
    return Ok(());
}
