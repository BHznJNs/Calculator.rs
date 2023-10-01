use crate::compiler::compile;
use crate::computer::compute;
use crate::public::run_time::scope::Scope;
use crate::public::value::Value;

pub fn attempt(input: &str, scope: &mut Scope) -> Result<Value, ()> {
    let root_node = compile(input)?;
    let result = compute(root_node, scope)?;
    return Ok(result);
}
