use std::rc::Rc;

use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::{array, invocation_params, assignment, object_reading};

pub fn resolve(
    var_node: ASTNode,
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    let is_more_token = tokens.len() > 0;

    let result_node =
    if is_more_token {
        let next_token = tokens.pop_front().unwrap();
        // let var_node_rc = Rc::new(var_node.clone());

        let current_node =
        match next_token {
            Token::Paren(Parens::LeftParen) => {
                // invocation for:
                // build-in function || lazy-expression ||
                // user-defined-function || object method.

                let mut invoke_params =
                    invocation_params::resolve(tokens)?;
                // if object-method, insert object variable as `self` param
                if let ASTNodeTypes::ObjectReading(node) =
                    &var_node.type__ {
                    if let ASTNodeTypes::Variable(obj_name) =
                        &node.type__ {
                            let self_node = ASTNode {
                                type__: ASTNodeTypes::Variable(obj_name.clone()),
                                params: None,
                            };
                            invoke_params.insert(0, ASTNode {
                                type__: ASTNodeTypes::Expression,
                                params: Some(vec![self_node]),
                            })
                    }
                }
                ASTNode {
                    type__: ASTNodeTypes::Invocation(Rc::new(var_node)),
                    params: Some(invoke_params),
                }
            },
            Token::Paren(Parens::LeftBracket) => {
                // array element reading
                array::reading_resolve(Rc::new(var_node), tokens)?
            },
            Token::Paren(_) => {
                // examples:
                // ...  PI)
                // ..., PI]
                tokens.push_front(next_token);
                return Ok(var_node)
            },
            Token::Symbol(Symbols::ObjectReading) => {
                // object property / method reading
                object_reading::resolve(Rc::new(var_node), tokens)?
            },
            Token::Symbol(symbol) => {
                if Symbols::is_equal_symbol(symbol) {
                    // assignment
                    assignment::resolve(
                        tokens,
                        symbol,
                        var_node
                    )?
                } else {
                    // next_token is symbol: + - * /
                    tokens.push_front(next_token);
                    return Ok(var_node)
                }
            },
            _ => {
                println!("Unexpected token: '{}'.", next_token);
                return Err(())
            }
        };
        resolve(current_node, tokens)?
    } else {
        var_node
    };

    Ok(result_node)
}