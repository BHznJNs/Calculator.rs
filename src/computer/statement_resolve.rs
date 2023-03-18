use crate::public::ast::{ASTNode, ASTNodeTypes};
use crate::public::global::Global;
use crate::public::keywords::Keyword;

use super::expression_compute::expression_compute;

pub fn statement_resolve(
    keyword: Keyword,
    statement_node: &ASTNode,
    global: &mut Global
) -> Result<(), ()> {
    let mut params =
    if let Some(ast_nodes) = &statement_node.params {
        ast_nodes.to_owned()
    } else {
        println!("Analyzer error.");
        return Err(())
    };

    // let params = statement_node.params.as_mut().unwrap();
    match keyword {
        Keyword::Out => {
            let expression_node = &params[0];
            let expression_res =
                expression_compute(expression_node, global)?;
            println!("{}", expression_res);
            Ok(())
        },
        Keyword::Loop => {
            let loop_count_expressiom = params.remove(0);
            let loop_count_struct =
                expression_compute(&loop_count_expressiom, global)?;
            let loop_count =
            match loop_count_struct {
                crate::public::number::Number::Empty => 0,
                crate::public::number::Number::NotANumber => 0,
                crate::public::number::Number::Int(i) => i,
                crate::public::number::Number::Float(f) => f as i64,
            };

            let mut index = 0;
            while index < loop_count {
                for sequence in &params {
                    match sequence.type__ {
                        ASTNodeTypes::Comment => {},
                        ASTNodeTypes::Expression => {
                            expression_compute(sequence, global)?;
                        },
                        ASTNodeTypes::Statement(keyword) => {
                            statement_resolve(keyword, sequence, global)?;
                        },
                        _ => {}
                    }
                }
                index += 1;
            }
            Ok(())
        },
    }
}