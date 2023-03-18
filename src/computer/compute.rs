use crate::public::ast::{ASTNode, ASTNodeTypes};
use crate::public::global::Global;
use crate::public::number::Number;

use super::expression_compute::expression_compute;
use super::statement_resolve::statement_resolve;

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

    match param_element.type__ {
        ASTNodeTypes::Expression => {
            let expression_res = expression_compute(
                param_element,
                global,
            )?;
            Ok(expression_res)
        },
        ASTNodeTypes::Statement(keyword) => {
            statement_resolve(
                keyword,
                param_element,
                global,
            )?;
            Ok(Number::Empty)
        },
        ASTNodeTypes::Comment => Ok(Number::Empty),
        _ => Err(())
    }
}