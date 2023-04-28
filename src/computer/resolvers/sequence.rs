use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{expression, statement};

pub fn resolve(
    sequence_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match sequence_node.type__ {
        ASTNodeTypes::Expression =>
            expression::resolve(sequence_node, scope)?,
        ASTNodeTypes::Statement(_) =>
            statement::resolve(sequence_node, scope)?,
        _ => Value::empty()
    };

    Ok(result)
}