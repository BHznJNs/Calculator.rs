use std::rc::Rc;

use crate::computer::resolvers::expression;
use crate::public::compile_time::ast::types::StatementNode;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::keywords::Keywords;
use crate::public::error::{syntax_error, import_error};
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::sequence;

pub fn resolve(
    statement_node: &mut StatementNode,
    scope: &mut Scope
) -> Result<Value, ()> {
    let (condition, body) =
        (statement_node.condition.take(), &statement_node.body);

    let result = match statement_node.keyword {
        Keywords::Out => {
            let output_value =
            if let ASTNode::Expression(expression_node) = &body[0] {
                expression::resolve(&expression_node, scope)?
            } else {
                Value::empty()
            };
            println!("{}", output_value);
            Value::empty()
        },
        Keywords::For => {
            let loop_count_expressiom =
                condition.unwrap();
            let loop_count_value =
                expression::resolve(&loop_count_expressiom, scope)?;

            let is_inf_loop;
            let loop_count = match loop_count_value {
                Value::Number(num) => {
                    is_inf_loop =
                        num == Number::Empty;
                    num.int_value()
                },
                _ => return Err(syntax_error("invalid loop count for 'for' statement")?)
            };

            let mut count = 0;
            loop {
                // these is used to control loop times
                if !is_inf_loop {
                    if count == loop_count {
                        break;
                    }
                    count += 1;
                }

                // --- --- --- --- --- ---

                let mut is_ended = false;

                for mut sequence in body {
                    let sequence_result =
                        sequence::resolve(&mut sequence, scope)?;

                    if let Value::Void(_) = sequence_result {
                        // encount `break` | `brk`
                        is_ended = true;
                        break;
                    }

                    if let Value::Void(None) = sequence_result {
                        // encount `continue` | `ctn`
                        break;
                    }
                }

                if is_ended {
                    break;
                }
            }

            Value::empty()
        },
        Keywords::If => {
            let condition_struct =
                expression::resolve(&condition.unwrap(), scope)?;
            let condition_value = match condition_struct {
                Value::Number(num) => num.int_value(),
                _ => return Err(syntax_error("invalid condition for 'if' statement")?)
            };

            if condition_value == 1 {
                for sequence in body {
                    let sequence_result =
                        sequence::resolve(&mut &sequence, scope)?;

                    if let Value::Void(_) = sequence_result {
                        return Ok(sequence_result)
                    }
                }
            }

            Value::empty()
        },

        Keywords::Import => {
            let module_node = &body[0];
            if let ASTNode::Variable(var_node) = module_node {
                scope.import_std(&var_node.name)?;
            } else
            if let ASTNode::StringLiteral(module_path) = module_node {
                scope.import_from_path(&module_path)?;
            } else {
                return Err(import_error("invalid import statement for wrong param type")?)
            }

            Value::empty()
        },

        Keywords::Break => {
            let ASTNode::Expression(expression_node) = &body[0] else {
                return Err(syntax_error("invalid return statement, expected expression returned")?)
            };
            let expression_res =
                expression::resolve(expression_node, scope)?;
            Value::Void(Some(Rc::new(expression_res)))
        },

        Keywords::Continue =>
            Value::Void(None),

        _ => Value::empty(),
    };

    Ok(result)
}