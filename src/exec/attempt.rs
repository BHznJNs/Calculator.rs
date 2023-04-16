use crate::compiler::compile::compile;
use crate::computer::compute::compute;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn attempt(
    input: &String,
    scope: &mut Scope
) -> Result<Value, ()> {
    let root_node = compile(input)?;
    // println!("{}", root_node); // LOG
    let result = compute(root_node, scope)?;

    Ok(result)
}