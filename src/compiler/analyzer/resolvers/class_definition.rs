use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::keywords::Keywords;
use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;

use super::function_definition;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    // no `cls` keyword

    if tokens.len() == 0 {
        println!("Invalid function definition.");
        return Err(())
    }
    let mut class_params = ASTNodeVec::new();
    let first_token =
        tokens.pop_front();

    if first_token == Some(Token::Paren(Parens::LeftBrace)) {
        let first_index = 0;
        while first_index < tokens.len() {
            let current = tokens.pop_front().unwrap();

            if let Token::Identi(prop) = current {
                let option_next_token = tokens.pop_front();
                let next_token = if option_next_token.is_some() {
                    option_next_token.unwrap()
                } else {
                    println!("Unmatched brace.");
                    return Err(())
                };

                match next_token {
                    Token::Divider | Token::Paren(Parens::RightBrace) => {
                        // class property
                        tokens.push_front(next_token);
                        class_params.push(ASTNode {
                            type__: ASTNodeTypes::Variable(prop),
                            params: None,
                        });
                    },
                    Token::Symbol(Symbols::Equal) => {
                        let next_token = tokens.pop_front();
                        // class method
                        if next_token == Some(Token::Keywords(Keywords::Function)) {
                            let method_node =
                                function_definition::resolve(tokens)?;
                            class_params.push(ASTNode {
                                type__: ASTNodeTypes::Variable(prop),
                                params: Some(vec![method_node]),
                            });
                        }
                    },
                    _ => {
                        println!("Unexpected token in class-definition body.");
                        return Err(())
                    }
                }
            } else if current == Token::Divider {
                continue;
            } else if current == Token::Paren(Parens::RightBrace) {
                break;
            } else {
                println!("Unexpected token in class-definition body.");
                return Err(())
            }
        }
    } else {
        println!("Expected class-definition body, which is missing.");
        return Err(())
    }
    Ok(ASTNode {
        type__: ASTNodeTypes::ClassDefinition,
        params: Some(class_params),
    })
}