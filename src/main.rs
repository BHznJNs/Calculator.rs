mod public;
mod compiler;
mod computer;

use std::collections::HashMap;
use std::io::{self, Write};

use public::number::Number;
use public::build_in::BuildIn;
use compiler::compile::compile;
use computer::compute::compute;
use computer::pre_compute::pre_compute;

fn attempt(input: String, build_in_funcs: &HashMap<&str, fn(f64) -> f64>) -> Result<Number, ()> {
    let tokens = compile(input)?;
    let resolved_tokens = pre_compute(tokens, build_in_funcs)?;
    let result = compute(resolved_tokens, build_in_funcs)?;
    Ok(result)
}

fn main() -> ! {
    let build_in_inst = BuildIn::init();
    let build_in_funcs = HashMap::from([
        ("sin", build_in_inst.sin),
        ("cos", build_in_inst.cos),
        ("tan", build_in_inst.tan)
    ]);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(input, &build_in_funcs);
        if result.is_ok() {
            println!(" {}", result.unwrap());
        }
    }
}