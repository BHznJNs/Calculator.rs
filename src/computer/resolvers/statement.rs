use std::rc::Rc;

use crate::computer::resolvers::expression;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::compile_time::keywords::Keywords;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::sequence;

pub fn resolve(
    statement_node: &ASTNode,
    scope: &mut Scope
) -> Result<Value, ()> {
    let ASTNodeTypes::Statement(keyword) = statement_node.type__ else {
        // msg for debug
        println!("Sequence_resolver error.");
        return Err(())
    };
    let params =
        statement_node
        .params.as_ref().unwrap();

    let result = match keyword {
        Keywords::Out => {
            let expression_node = &params[0];
            // compute value of the expression after `out`
            let expression_res =
                expression::resolve(expression_node, scope)?;
            // do not print empty sequence
            if expression_res != Value::empty() {
                println!("{}", expression_res);
            }
            expression_res
        },
        Keywords::For => {
            let loop_count_expressiom = &params[0];
            let loop_count_struct =
                expression::resolve(&loop_count_expressiom, scope)?;
            
            let is_inf_loop;
            let loop_count = match loop_count_struct {
                Value::Number(num) => {
                    is_inf_loop =
                        num == Number::Empty;
                    num.int_value()
                },
                _ => {
                    println!("Invalid loop count for 'for' statement");
                    return Err(())
                }
            };

            let mut count = 0;
            let loop_body = &params[1..];
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

                for sequence in loop_body {
                    let sequence_result =
                        sequence::resolve(sequence, scope)?;

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
            let condition = &params[0];
            let condition_struct =
                expression::resolve(&condition, scope)?;
            let condition_value = match condition_struct {
                Value::Number(num) => num.int_value(),
                _ => {
                    println!("Invalid condition for 'if' statement.");
                    return Err(())
                }
            };

            if condition_value == 1 {
                let condition_body = &params[1..];
                for sequence in condition_body {
                    let sequence_result =
                        sequence::resolve(sequence, scope)?;

                    if let Value::Void(_) = sequence_result {
                        return Ok(sequence_result)
                    }
                }
            }

            Value::empty()
        },

        Keywords::Import => {
            let module_node = &params[0];
            if let ASTNodeTypes::Variable(module_name) = &module_node.type__ {
                scope.import(module_name)?;
            } else
            if let ASTNodeTypes::StringLiteral(module_path) = &module_node.type__ {
                scope.import_from_path(module_path)?;
            } else {
                println!("Analyzer error: invalid module type.");
                return Err(())
            }
            
            Value::empty()
        },

        Keywords::Break => {
            let expression_node = &params[0];
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