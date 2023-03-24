use crate::compiler::compile;
use crate::computer::compute;
use crate::public::run_time::global::Global;
use crate::public::value::value::Value;

pub fn attempt(
    input: String, global: &mut Global
) -> Result<Value, ()> {
    let root_node = compile::compile(input)?;
    // println!("{}", root_node); // LOG
    let result_num = compute::compute(root_node, global)?;

    Ok(result_num)
}