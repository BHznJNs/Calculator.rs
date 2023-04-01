use crate::public::compile_time::ast::{ASTNode, ASTNodeVec, ASTNodeTypes};
use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::value::function::Param;
use crate::public::value::parens::Parens;

use super::statement_resolve::statement_body_resolve;

fn func_params_resolve(
    tokens: &mut TokenVec
) -> Result<Vec<Param>, ()> {
    // return function Vec<Param>
    // structure:
    // identi annotation) {function body ...}

    let first_index = 0;
    let mut params = Vec::<Param>::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();

        if tokens.len() >= 3 {
            // has more param
            match current {
                Token::Identi(identi) => {
                    let next = tokens.pop_front().unwrap();
                    if let Token::Annotation(type__) = next {
                        params.push(Param { type__, identi })
                    } else {
                        println!("Invalid function definition: expected type annotation.");
                        return Err(())
                    }
                },
                Token::Divider => continue,
                Token::Paren(Parens::RightParen) => break,
                _ => {
                    println!("Unexpected token: {} in function definition.", current);
                    return Err(())
                }
            }
        } else if tokens.len() > 0 {
            if current == Token::Paren(Parens::RightParen) {
                break;
            }
        } else {
            println!("Invalid function definition.");
            return Err(())
        }
    }

    Ok(params)
}

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    // no `fn` keyword

    if tokens.len() == 0 {
        println!("Invalid function definition.");
        return Err(())
    }
    let first_token = tokens.pop_front().unwrap();
    if first_token == Token::Paren(Parens::LeftParen) {
        let func_params =
            func_params_resolve(tokens)?;

        let next_token = tokens.pop_front();
        if next_token != Some(Token::Paren(Parens::LeftBrace)) {
            println!("Expected function body, which is missing.");
            return Err(())
        }

        let mut func_body_params = ASTNodeVec::new();
        statement_body_resolve(tokens, &mut func_body_params)?;

        Ok(ASTNode {
            type__: ASTNodeTypes::FunctionDefinition(func_params),
            params: Some(func_body_params),
        })
    } else {
        println!("Expected token: LeftParen '(' for function definition.");
        return Err(())
    }
}