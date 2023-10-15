use std::borrow::Borrow;

use super::expression;
use crate::public::compile_time::ast::types::ArrayLiteralNode;
use crate::public::error::CalcResult;
use crate::public::run_time::scope::Scope;
use crate::public::value::array::ArrayLiteral;

pub fn resolve(node: &ArrayLiteralNode, scope: &mut Scope) -> CalcResult<ArrayLiteral> {
    let mut elements = ArrayLiteral::new();

    for element in &node.elements {
        let expression_value = expression::resolve(element.borrow(), scope)?;
        elements.push_back(expression_value)
    }

    return Ok(elements);
}
