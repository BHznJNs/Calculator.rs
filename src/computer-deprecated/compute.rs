use crate::public::compile_time::ast::ASTNode;
use crate::public::run_time::global::Global;
use crate::public::value::value::Value;

use super::resolvers::sequence_resolve::sequence_resolve;

pub fn compute(
    root_node: ASTNode,
    global: &mut Global,
) -> Result<Box<Value>, ()> {
    /*
        Root {
          Expression {
            Assignment,
            Symbol,
            Number,
            Symbol,
            Expression,
            ...
          }
        }
     */

    let params = root_node
        .params
        .as_ref()
        .unwrap();

    let param_element = &params[0];
    let result =
        sequence_resolve(param_element, global)?;

    Ok(result)
}