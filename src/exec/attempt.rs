use std::collections::HashMap;

use crate::compiler::compile;
use crate::computer::compute;
use crate::public::number::Number;
use crate::public::ast::ASTNode;

pub fn attempt(
    input: String,
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>,
) -> Result<Number, ()> {
    let root_node = compile::compile(input)?;
    let result_num = compute::compute(
        root_node,
        build_in_funcs,
        variables,
        goto_statements,
    )?;

    Ok(result_num)
}