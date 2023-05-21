use std::rc::Rc;

use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::public::{compile_time::ast::ast_enum::ASTNode, value::value::VoidSign};

use super::{expression, statement};

pub fn resolve(sequence_node: Rc<ASTNode>, scope: &mut Scope) -> Result<Value, ()> {
    let result = match sequence_node.as_ref() {
        ASTNode::Expression(expression_node) => {
            expression::resolve(expression_node.clone(), scope)?
        }
        ASTNode::Statement(statement_node) => statement::resolve(statement_node.clone(), scope)?,
        _ => Value::Void(VoidSign::Empty),
    };

    Ok(result)
}
