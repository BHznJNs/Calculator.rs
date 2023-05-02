use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::{Value, Overload};

use super::assignment;
use super::{array_literal, function_definition, instantiation, compose::compose};
use super::operate::operate;

pub fn resolve(
    expression_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let params = expression_node
        .params
        .as_ref()
        .unwrap();
    // empty expression
    if params.len() == 0 {
        return Ok(Value::empty())
    }

    let mut value_stack = Vec::<Value>::new();

    for node in params {
        let current_value =
        match &node.type__ {
            ASTNodeTypes::Expression =>
                resolve(node, scope)?,


            ASTNodeTypes::NumberLiteral(num) => 
                Value::Number(*num),
            ASTNodeTypes::StringLiteral(str) =>
                Value::create(str.to_owned()),

            ASTNodeTypes::LazyExpression =>
                Value::create(node.to_owned()),
            ASTNodeTypes::FunctionDefinition(_) =>
                Value::create(function_definition::resolve(node)?),

            ASTNodeTypes::SymbolLiteral(symbol) => {
                if *symbol == Symbols::Not {
                    if let Some(val) = value_stack.pop() {
                        let Value::Number(num) = val else {
                            println!("Invalid operating value type, expected Number.");
                            return Err(())
                        };
                        Value::Number(num.not())
                    } else {
                        println!("Operating value is missing for Not operator.");
                        return Err(())
                    }
                } else {
                    if value_stack.len() < 2 {
                        // no enough value for operating
                        println!("Invalid expression: operating number is missing.");
                        return Err(())
                    }
                    let num2 = value_stack.pop().unwrap();
                    let num1 = value_stack.pop().unwrap();
                    let current_symbol = *symbol;
                    operate(num1, num2, current_symbol)?
                }
                
            },
            ASTNodeTypes::ArrayLiteral => {
                let array_elements =
                    array_literal::resolve(node, scope)?;
                Value::create(array_elements)
            },
            ASTNodeTypes::Instantiation(_) =>
                Value::create(instantiation::resolve(node, scope)?),
            
            ASTNodeTypes::Variable(_) |
            ASTNodeTypes::ObjectReading(_) |
            ASTNodeTypes::Invocation(_) |
            ASTNodeTypes::ArrayElementReading(_) =>
                compose::resolve(node, scope)?,

            ASTNodeTypes::Assignment(_) =>
                assignment::resolve(node, scope)?,
            _ => {
                println!("Unexpected node type: '{}'.", node.type__);
                return Err(())
            }
        };
        value_stack.push(current_value);
    }
    Ok(value_stack.remove(0))
}