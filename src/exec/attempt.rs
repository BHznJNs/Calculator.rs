use crate::compiler::compile;
use crate::computer::compute;
use crate::public::global::Global;
use crate::public::number::Number;

pub fn attempt(
    input: String, global: &mut Global
) -> Result<Number, ()> {
    let root_node = compile::compile(input)?;
    // println!("{}", root_node); // LOG
    let result_num = compute::compute(root_node, global)?;

    Ok(result_num)
}