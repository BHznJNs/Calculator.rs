use std::rc::Rc;

use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::error::{syntax_error, type_error, internal_error, InternalComponent};
use crate::public::run_time::scope::Scope;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::{Value, Overload, ValueType, VoidSign};

use super::class_definition;
use super::{function_definition, array_literal, instantiation, assignment, composer::compose};
use super::operate::operate;

pub fn resolve(
    node:  Rc<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let elements =
        &node.elements;

    if elements.len() == 0 {
        return Ok(Value::Void(VoidSign::Empty))
    }

    let mut value_stack = Vec::<Value>::new();

    for current_node in elements {
        let current_value =
        match current_node {
            ASTNode::Expression(node) =>
                resolve(node.clone(), scope)?,

            ASTNode::NumberLiteral(num) => 
                Value::Number(*num),
            ASTNode::StringLiteral(str) =>
                Value::create(str.to_owned()),

            ASTNode::LazyExpression(node) =>
                Value::LazyExpression(node.sub_sequence.clone().into()),
            ASTNode::FunctionDefinition(node) =>
                Value::create(function_definition::resolve(node.clone())?),
            ASTNode::ClassDefinition(node) =>
                Value::create(class_definition::resolve(node.clone())?),

            ASTNode::SymbolLiteral(sym) => {
                if *sym == Symbols::Not {
                    if let Some(val) = value_stack.pop() {
                        let Value::Number(num) = val else {
                            return Err(type_error(
                                Some("Not operator"),
                                ValueType::Number,
                                val.get_type(),
                            )?)
                        };
                        Value::Number(num.not())
                    } else {
                        return Err(syntax_error("operating number is missing for Not operator")?)
                    }
                } else {
                    if value_stack.len() < 2 {
                        // no enough value for operating
                        return Err(syntax_error("invalid expression as operating number missing")?)
                    }

                    let num2 = value_stack.pop().unwrap();
                    let num1 = value_stack.pop().unwrap();
                    let current_symbol = *sym;
                    operate(num1, num2, current_symbol)?
                }
            },
            ASTNode::ArrayLiteral(node) => {
                let array_elements =
                    array_literal::resolve(node.clone(), scope)?;
                Value::create(array_elements)
            },
            ASTNode::Instantiation(node) =>
                Value::create(instantiation::resolve(node.clone(), scope)?),
            ASTNode::Assignment(node) =>
                assignment::resolve(node.clone(), scope)?,

            ASTNode::Variable(_) |
            ASTNode::ObjectReading(_) |
            ASTNode::Invocation(_) |
            ASTNode::ArrayElementReading(_) =>
                compose::resolve(current_node.clone().into(), scope)?,

            _ => {
                let msg = format!("unexpected AST node: '{}'", current_node);
                return Err(internal_error(InternalComponent::Analyzer, &msg)?)
            }
        };
        value_stack.push(current_value);
    }
    Ok(value_stack.remove(0))
}