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

fn build_in_funcs_init(build_in_inst: &BuildIn) -> HashMap<&str, fn(f64) -> f64> {
    let map = HashMap::from([
        ("sin" , build_in_inst.sin),
        ("cos" , build_in_inst.cos),
        ("tan" , build_in_inst.tan),
        ("asin", build_in_inst.asin),
        ("acos", build_in_inst.acos),
        ("atan", build_in_inst.atan),
        ("sinh", build_in_inst.sinh),
        ("cosh", build_in_inst.cosh),
        ("tanh", build_in_inst.tanh),

        ("rad", build_in_inst.rad),
        ("deg", build_in_inst.deg),

        ("log10", build_in_inst.log10),
        ("log2" , build_in_inst.log2),
        ("ln"   , build_in_inst.ln),
        ("exp"  , build_in_inst.exp),

        ("abs"  , build_in_inst.abs),
        ("sqrt" , build_in_inst.sqrt),
        ("floor", build_in_inst.floor),
        ("round", build_in_inst.round),
    ]);
    return map
}
fn build_in_const_init(build_in_inst: &BuildIn) -> HashMap<String, Number> {
    let map = HashMap::from([
        ("PI".to_string(), Number::Float(build_in_inst.pi)),
        ("E" .to_string(), Number::Float(build_in_inst.e )),
    ]);
    return map
}

fn attempt(
    input: String,
    variables:  &mut HashMap<String, Number>,
    build_in_funcs: &HashMap<&str, fn(f64) -> f64>,
) -> Result<Number, ()> {
    let tokens = compile(input)?;
    let resolved_tokens = pre_compute(tokens, variables, build_in_funcs)?;
    let result = compute(resolved_tokens, variables, build_in_funcs)?;
    Ok(result)
}

fn main() -> ! {
    let build_in_inst = BuildIn::init();
    let build_in_funcs = build_in_funcs_init(&build_in_inst);
    let mut variables = build_in_const_init(&build_in_inst);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();

        let result = attempt(
            input,
            &mut variables,
            &build_in_funcs,
        );
        if result.is_ok() {
            println!("= {}", result.unwrap());
        }
    }
}