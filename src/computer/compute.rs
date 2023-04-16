use crate::public::compile_time::ast::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::resolvers::sequence;

pub fn compute(
    root_node: ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    /*
      Root {
        Expression {
          Assignment,
          Symbol,
          Number,
          Symbol,
          Expression,
          LazyExpression,
          ...
        },
        Statement {
          Keywords,
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
        sequence::resolve(param_element, scope)?;

    Ok(result)
}