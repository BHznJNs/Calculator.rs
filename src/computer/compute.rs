use crate::public::ast::ASTNode;
use crate::public::global::Global;
use crate::public::value::number::Number;

use super::sequence_resolve::sequence_resolve;

pub fn compute(
    root_node: ASTNode,
    global: &mut Global,
) -> Result<Number, ()> {
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