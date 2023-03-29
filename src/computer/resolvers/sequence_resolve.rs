use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{expression_resolve, statement_resolve};

pub fn resolve(
    sequence_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let result = match sequence_node.type__ {
        ASTNodeTypes::Expression => {
            expression_resolve::resolve(sequence_node, scope)?
        },
        ASTNodeTypes::Statement(_) => {
            let optional_keyword =
                statement_resolve::resolve(sequence_node, scope)?;
            Value::empty(optional_keyword)
        },
        _ => Value::empty(None)
    };

    Ok(result)
}