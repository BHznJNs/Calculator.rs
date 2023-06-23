use crate::compiler::compile;
use crate::computer::computer::compute;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn attempt(input: &String, scope: &mut Scope) -> Result<Value, ()> {
    let root_node = compile(input)?;
    let result = compute(root_node, scope)?;

    Ok(result)
}
