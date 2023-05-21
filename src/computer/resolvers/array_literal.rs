use std::rc::Rc;

use super::expression;
use crate::public::compile_time::ast::types::ArrayLiteralNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::array::ArrayLiteral;

pub fn resolve(node: Rc<ArrayLiteralNode>, scope: &mut Scope) -> Result<ArrayLiteral, ()> {
    let mut elements = ArrayLiteral::new();

    for element in &node.elements {
        let expression_value = expression::resolve(element.clone().into(), scope)?;
        elements.push_back(expression_value)
    }

    Ok(elements)
}
