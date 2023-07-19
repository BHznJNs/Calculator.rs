use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::{ExpressionNode, ModuleType};
use crate::public::error::{internal_error, syntax_error, type_error, InternalComponent};
use crate::public::run_time::scope::Scope;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::class_definition;
use super::operate::operate;
use super::{array_literal, assignment, composer::compose, function_definition, instantiation};

pub fn resolve(node: &ExpressionNode, scope: &mut Scope) -> Result<Value, ()> {
    let elements = &node.elements;
    if elements.len() == 0 {
        return Ok(Value::Void(VoidSign::Empty));
    }

    let mut value_stack = Vec::<Value>::new();

    for current_node in elements {
        let current_value = match current_node {
            ASTNode::Expression(node) => resolve(node, scope)?,

            ASTNode::NumberLiteral(num) => Value::Number(num.clone()),
            ASTNode::StringLiteral(str) => Value::from(str.clone()),

            ASTNode::LazyExpression(node) => {
                Value::LazyExpression(node.sub_sequence.clone().into())
            }

            ASTNode::ImportStatement(node) => {
                if node.type__ == ModuleType::UserDefined {
                    scope.import_from_path(&node.target)?
                } else {
                    unreachable!()
                }
            }
            ASTNode::FunctionDefinition(node) => Value::from(function_definition::resolve(node)?),
            ASTNode::ClassDefinition(node) => Value::from(class_definition::resolve(node)?),

            ASTNode::SymbolLiteral(sym) => {
                if *sym == Symbols::Not {
                    // get last value and expected as Number | Boolean typed
                    if let Some(val) = value_stack.pop() {
                        if let Value::Number(num) = val {
                            Value::Number(num.not())
                        } else if let Value::Boolean(bool_val) = val {
                            Value::Boolean(!bool_val)
                        } else {
                            return Err(type_error(
                                Some("Not operator"),
                                vec![ValueType::Number],
                                val.get_type(),
                            )?);
                        }
                    } else {
                        return Err(syntax_error(
                            "operating number is missing for Not operator",
                        )?);
                    }
                } else {
                    if value_stack.len() < 2 {
                        // no enough value for operating
                        return Err(syntax_error(
                            "invalid expression as operating number missing",
                        )?);
                    }

                    let num2 = value_stack.pop().unwrap();
                    let num1 = value_stack.pop().unwrap();
                    let current_symbol = *sym;
                    operate(num1, num2, current_symbol)?
                }
            }
            ASTNode::ArrayLiteral(node) => Value::from(array_literal::resolve(node, scope)?),
            ASTNode::Instantiation(node) => Value::from(instantiation::resolve(node, scope)?),
            ASTNode::Assignment(node) => assignment::resolve(node, scope, false)?,

            ASTNode::Variable(_)
            | ASTNode::ObjectReading(_)
            | ASTNode::Invocation(_)
            | ASTNode::ArrayElementReading(_) => compose::resolve(current_node, scope)?,

            _ => {
                let msg = format!("unexpected AST node: '{}'", current_node);
                return Err(internal_error(InternalComponent::Analyzer, &msg)?);
            }
        };
        value_stack.push(current_value);
    }
    Ok(value_stack.remove(0))
}
