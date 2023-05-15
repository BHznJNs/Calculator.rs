use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::ast_enum::{ASTNode, ASTVec};
use crate::public::compile_time::ast::types::{ClassDefinitionNode, VariableNode};
use crate::public::compile_time::keywords::Keywords;
use crate::public::error::syntax_error;
use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;

use super::function_definition;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ClassDefinitionNode, ()> {
    // no `cls` keyword
    // example:
    // {prop, method=fn(){exec()}}

    if tokens.len() == 0 {
        return Err(syntax_error("missing class body")?)
    }
    let mut class_params = ASTVec::new();
    let first_token = tokens
        .pop_front()
        .unwrap();

    if first_token == Token::Paren(Parens::LeftBrace) {
        let first_index = 0;
        while first_index < tokens.len() {
            let current =
                tokens.pop_front().unwrap();

            if let Token::Identi(prop) = current {
                let Some(next_token) = tokens.pop_front() else {
                    // if no token follows the property
                    return Err(syntax_error("Unmatched brace")?)
                };

                match next_token {
                    Token::Divider | Token::Paren(Parens::RightBrace) => {
                        // current as class property
                        tokens.push_front(next_token);
                        class_params.push(ASTNode::Variable(VariableNode {
                            name: prop
                        }.into()));
                    },
                    Token::Symbol(Symbols::Equal) => {
                        let next_token = tokens.pop_front();
                        // current as class method
                        if next_token == Some(Token::Keywords(Keywords::Function)) {
                            let mut method_node =
                                function_definition::resolve(tokens)?;
                            method_node.name = Some(prop);

                            class_params.push(ASTNode::FunctionDefinition(
                                method_node.into()
                            ))
                        }
                    },
                    _ => {
                        let msg = format!("unexpected token {} in class body.", next_token);
                        return Err(syntax_error(&msg)?)
                    }
                }
            } else if current == Token::Divider {
                continue;
            } else if current == Token::Paren(Parens::RightBrace) {
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
        params: class_params,
    })
}