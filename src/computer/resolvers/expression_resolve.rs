use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, Overload};

use super::invocation::invocation_resolve;
use super::variable_reading::variable_reading;
use super::{array_resolve, array_reading_resolve};
use super::{assignment_resolve, operate::operate};

pub fn resolve(
    expression_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let params = expression_node
        .params
        .as_ref()
        .unwrap();
    // empty expression
    if params.len() == 0 {
        return Ok(Value::empty())
    }

    let mut value_stack = Vec::<Rc<Value>>::new();

    for node in params {
        match &node.type__ {
            ASTNodeTypes::Variable(name) =>
                value_stack.push(variable_reading(name, scope)?),
            ASTNodeTypes::Assignment(_) =>
                value_stack.push(assignment_resolve::resolve(node, scope)?),
            ASTNodeTypes::NumberLiteral(num) => 
                value_stack.push(Rc::new(Value::Number(*num))),
            ASTNodeTypes::StringLiteral(str) =>
                value_stack.push(Rc::new(Value::create(str.to_owned()))),
            ASTNodeTypes::SymbolLiteral(symbol) => {
                if value_stack.len() < 2 {
                    // no enough value for operating
                    println!("Invalid expression: operating number is missing.");
                    return Err(())
                }
                let num2 = value_stack.pop().unwrap();
                let num1 = value_stack.pop().unwrap();
                let current_symbol = *symbol;
                let value =
                    operate(num1, num2, current_symbol)?;

                value_stack.push(Rc::new(value));
            },
            ASTNodeTypes::ArrayLiteral => {
                let array_elements =
                    array_resolve::resolve(node, scope)?;
                let array_rc =
                    Rc::new(Value::create(array_elements));

                value_stack.push(array_rc);
            },
            ASTNodeTypes::ArrayElementReading(_) =>
                value_stack.push(array_reading_resolve::resolve(node, scope)?),
            ASTNodeTypes::Expression =>
                value_stack.push(resolve(node, scope)?),
            ASTNodeTypes::LazyExpression =>
                value_stack.push(Rc::new(Value::create(node.to_owned()))),
            ASTNodeTypes::Invocation(_) =>
                value_stack.push(invocation_resolve::resolve(node, scope)?),
            _ => {
                println!("Unexpected node type: '{}'.", node.type__);
                return Err(())
            }
        }
    }
    Ok(value_stack.remove(0))
}