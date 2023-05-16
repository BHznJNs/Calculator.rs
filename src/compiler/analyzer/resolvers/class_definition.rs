use std::rc::Rc;

use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::types::{ClassDefinitionNode, FunctionDefinitionNode};
use crate::public::compile_time::keywords::Keywords;
use crate::public::error::syntax_error;
use crate::public::compile_time::parens::Paren;
use crate::public::value::oop::class::Property;
use crate::public::value::symbols::Symbols;

use super::function_definition;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ClassDefinitionNode, ()> {
    // no `cls` keyword
    // example:
    // { prop $_, method=fn(){do something...} }

    if tokens.len() == 0 {
        return Err(syntax_error("missing class body")?)
    }

    let mut properties =
        Vec::<Property>::new();
    let mut method_nodes =
        Vec::<Rc<FunctionDefinitionNode>>::new();

    let first_token = tokens
        .pop_front()
        .unwrap();

    if first_token == Token::Paren(Paren::LeftBrace) {
        loop {
            if tokens.len() == 0 {
                return Err(syntax_error("unmatched brace")?)
            }

            let current =
                tokens.pop_front().unwrap();

            if let Token::Identi(identi) = current {
                let Some(next_token) = tokens.pop_front() else {
                    // if no token follows the property
                    return Err(syntax_error("unmatched brace")?)
                };

                match next_token {
                    Token::Annotation(type__) => {
                        properties.push(Property {
                            type__, identi,
                        })
                    },
                    Token::Symbol(Symbols::Equal) => {
                        let next_token =
                            tokens.pop_front();
                        // current as class method
                        if next_token == Some(Token::Keywords(Keywords::Function)) {
                            let mut method_node =
                                function_definition::resolve(tokens)?;
                            method_node.name = Some(identi);

                            method_nodes.push(method_node.into())
                        }
                    },
                    _ => {
                        let msg = format!("unexpected token {} in class body.", next_token);
                        return Err(syntax_error(&msg)?)
                    }
                }
            } else if current == Token::Divider {
                continue;
            } else if current == Token::Paren(Paren::RightBrace) {
                break;
            } else {
                let msg = format!("unexpected token {} in class body.", current);
                return Err(syntax_error(&msg)?)
            }
        }
    } else {
        return Err(syntax_error("expected class-definition body")?)
    }
    Ok(ClassDefinitionNode {
        properties,
        method_nodes,
    })
}