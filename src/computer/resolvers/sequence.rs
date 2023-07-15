use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::public::{compile_time::ast::ast_enum::ASTNode, value::value::VoidSign};

use super::{expression, statement};

pub fn resolve(sequence_node: &ASTNode, scope: &mut Scope) -> Result<Value, ()> {
    let result = match sequence_node {
        ASTNode::Expression(expression_node) => expression::resolve(expression_node, scope)?,
        ASTNode::Statement(statement_node) => statement::resolve(statement_node, scope)?,
        _ => Value::Void(VoidSign::Empty),
    };

    return Ok(result); 
}
