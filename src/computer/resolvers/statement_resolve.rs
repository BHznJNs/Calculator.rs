use crate::public::compile_time::ast::ASTNode;
use crate::public::compile_time::keywords::Keyword;
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::super::expression_compute::expression_compute;
use super::sequence_resolve::sequence_resolve;

pub fn statement_resolve(
    keyword: Keyword,
    statement_node: &ASTNode,
    global: &mut Global
) -> Result<Option<Keyword>, ()> {
    let params =
    if let Some(ast_nodes) = &statement_node.params {
        ast_nodes
    } else {
        println!("Analyzer error from 'statement_resolve'.");
        return Err(())
    };

    match keyword {
        Keyword::Out => {
            let expression_node = &params[0];
            let expression_res =
                expression_compute(expression_node, global)?;
            println!("{}", expression_res);
            Ok(None)
        },
        Keyword::For => {
            let loop_count_expressiom = &params[0];
            let loop_count_struct =
                expression_compute(&loop_count_expressiom, global)?;
            let loop_count = match loop_count_struct {
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
                        sequence_resolve(sequence, global)?;
                    
                    if sequence_result ==
                       Value::Number(Number::Empty(Some(Keyword::Continue))) {
                        break;
                    }
                    if sequence_result ==
                       Value::Number(Number::Empty(Some(Keyword::Break))) {
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
        Keyword::If => {
            let condition = &params[0];
            let condition_struct =
                expression_compute(&condition, global)?;
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
                        sequence_resolve(sequence, global)?;

                    if let Value::Number(Number::Empty(keyword)) =
                           sequence_result {
                        return Ok(keyword)
                    }
                }
            }

            Ok(None)
        },
        Keyword::Continue => Ok(Some(keyword)),
        Keyword::Break    => Ok(Some(keyword)),
    }
}