use crate::public::compile_time::ast::ASTNode;
use crate::public::compile_time::keywords::Keyword;
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;

use super::super::expression_compute::expression_compute;
use super::sequence_resolve::sequence_resolve;

pub fn statement_resolve(
    keyword: Keyword,
    statement_node: &ASTNode,
    global: &mut Global
) -> Result<Option<Keyword>, ()> {
    let mut params =
    if let Some(ast_nodes) = &statement_node.params {
        ast_nodes.to_owned()
    } else {
        println!("Analyzer error.");
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
            let loop_count_expressiom = params.remove(0);
            let loop_count_struct =
                expression_compute(&loop_count_expressiom, global)?;
            let loop_count = loop_count_struct.int_value();

            for _ in 0..loop_count {
                let mut is_broken = false;

                for sequence in &params {
                    let sequence_result =
                        sequence_resolve(sequence, global)?;
                    
                    if sequence_result == Number::Empty(Some(Keyword::Continue)) {
                        break;
                    }
                    if sequence_result == Number::Empty(Some(Keyword::Break)) {
                        is_broken = true;
                        break;
                    }
                }

                if is_broken {
                    break;
                }
            }

            Ok(None)
        },
        Keyword::If => {
            let condition = params.remove(0);
            let condition_struct =
                expression_compute(&condition, global)?;
            let condition_value = condition_struct.int_value();

            if condition_value == 1 {
                for sequence in &params {
                    let sequence_result =
                        sequence_resolve(sequence, global)?;

                    if let Number::Empty(keyword) = sequence_result {
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