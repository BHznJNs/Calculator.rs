use crate::computer::resolvers::expression_resolve;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::compile_time::keywords::Keywords;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::sequence_resolve;

pub fn resolve(
    statement_node: &ASTNode,
    scope: &mut Scope
) -> Result<Option<Keywords>, ()> {
    let ASTNodeTypes::Statement(keyword) = statement_node.type__ else {
        println!("Sequence_resolver error.");
        return Err(())
    };
    let params =
        statement_node
        .params.as_ref().unwrap();

    match keyword {
        Keywords::Out => {
            let expression_node = &params[0];
            let expression_res =
                expression_resolve::resolve(expression_node, scope)?;
            println!("{}", expression_res);
            Ok(None)
        },
        Keywords::Fn => {
            Ok(None)
        },
        Keywords::For => {
            let loop_count_expressiom = &params[0];
            let loop_count_struct =
                expression_resolve::resolve(&loop_count_expressiom, scope)?;
            let loop_count = match *loop_count_struct {
                Value::Number(num) => num.int_value(),
                _ => {
                    println!("Invalid loop count for 'for' statement");
                    return Err(())
                }
            };

            for _ in 0..loop_count {
                let mut is_ended = false;
                let loop_body = &params[1..];

                for sequence in loop_body {
                    let sequence_result =
                        sequence_resolve::resolve(sequence, scope)?;

                    if *sequence_result ==
                       Value::Number(Number::Empty(Some(Keywords::Continue))) {
                        // encount `continue` | `ctn`
                        break;
                    }
                    if *sequence_result ==
                       Value::Number(Number::Empty(Some(Keywords::Break))) {
                        // encount `break` | `brk`
                        is_ended = true;
                        break;
                    }
                }

                if is_ended {
                    break;
                }
            }

            Ok(None)
        },
        Keywords::If => {
            let condition = &params[0];
            let condition_struct =
                expression_resolve::resolve(&condition, scope)?;
            let condition_value = match *condition_struct {
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
                        sequence_resolve::resolve(sequence, scope)?;

                    if let Value::Number(Number::Empty(keyword)) =
                           *sequence_result {
                        return Ok(keyword)
                    }
                }
            }

            Ok(None)
        },

        Keywords::Import => {
            let module_node = &params[0];
            let ASTNodeTypes::Variable(module_name) =
                &module_node.type__ else {
                println!("Analyzer error: invalid module type.");
                return Err(())
            };

            scope.import(module_name)?;
            Ok(None)
        },

        Keywords::Continue => Ok(Some(keyword)),
        Keywords::Break    => Ok(Some(keyword)),
    }
}