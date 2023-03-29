use crate::public::compile_time::ast::ASTNode;
use crate::public::compile_time::keywords::Keywords;
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::super::expression_compute::expression_compute;
use super::sequence_resolve::sequence_resolve;

pub fn statement_resolve(
    keyword: Keywords,
    statement_node: &ASTNode,
    global: &mut Global
) -> Result<Option<Keywords>, ()> {
    let params =
    if let Some(ast_nodes) = &statement_node.params {
        ast_nodes
    } else {
        println!("Analyzer error from 'statement_resolve'.");
        return Err(())
    };

    match keyword {
        Keywords::Out => {
            let expression_node = &params[0];
            let expression_res =
                expression_compute(expression_node, global)?;
            println!("{}", expression_res);
            Ok(None)
        },
        Keywords::For => {
            let loop_count_expressiom = &params[0];
            let loop_count_struct =
                expression_compute(&loop_count_expressiom, global)?;
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
                        *sequence_resolve(sequence, global)?;

                    if sequence_result ==
                       Value::Number(Number::Empty(Some(Keywords::Continue))) {
                        break;
                    }
                    if sequence_result ==
                       Value::Number(Number::Empty(Some(Keywords::Break))) {
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
                expression_compute(&condition, global)?;
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
                        sequence_resolve(sequence, global)?;

                    if let Value::Number(Number::Empty(keyword)) =
                           *sequence_result {
                        return Ok(keyword)
                    }
                }
            }

            Ok(None)
        },
        Keywords::Continue => Ok(Some(keyword)),
        Keywords::Break    => Ok(Some(keyword)),
    }
}