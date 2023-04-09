use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, Overload};

use super::invocation::invocation_resolve;
use super::{array_literal, array_reading, variable_reading, function_definition, object_reading, instantiation};
use super::{assignment, operate::operate};

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
        let current_value =
        match &node.type__ {
            ASTNodeTypes::Variable(name) =>
                variable_reading::resolve(name, scope)?,
            ASTNodeTypes::Assignment(_) =>
                assignment::resolve(node, scope)?,
            ASTNodeTypes::NumberLiteral(num) => 
                Rc::new(Value::Number(*num)),
            ASTNodeTypes::StringLiteral(str) =>
                Value::create_rc(str.to_owned()),

            ASTNodeTypes::ArrayElementReading(_) =>
                array_reading::resolve(node, scope)?,
            ASTNodeTypes::Expression =>
                resolve(node, scope)?,
            ASTNodeTypes::LazyExpression =>
                Value::create_rc(node.to_owned()),
            ASTNodeTypes::Invocation(_) =>
                invocation_resolve::resolve(node, scope)?,
            ASTNodeTypes::FunctionDefinition(_) =>
                Value::create_rc(function_definition::resolve(node)?),

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

                Rc::new(value)
            },
            ASTNodeTypes::ArrayLiteral => {
                let array_elements =
                    array_literal::resolve(node, scope)?;
                let array_rc =
                    Value::create_rc(array_elements);

                array_rc
            },
            ASTNodeTypes::Instantiation(_) =>
                Value::create_rc(instantiation::resolve(node, scope)?),
            ASTNodeTypes::ObjectReading(_) =>
                object_reading::resolve(node, scope)?,
            _ => {
                println!("Unexpected node type: '{}'.", node.type__);
                return Err(())
            }
        };
        value_stack.push(current_value);
    }
    Ok(value_stack.remove(0))
}