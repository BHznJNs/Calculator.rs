use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::types::FunctionDefinitionNode;
use crate::public::error::syntax_error;
use crate::public::value::function::Param;
use crate::public::value::parens::Parens;

use super::statement_block;

// refactor: params_resolve
fn params_resolve(
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
                        return Err(syntax_error("type annotation expected in function definition")?)
                    }
                },
                Token::Divider => continue,
                Token::Paren(Parens::RightParen) => break,
                _ => {
                    let msg = format!("unexpected token {} in function param", current);
                    return Err(syntax_error(&msg)?)
                }
            }
        } else if tokens.len() > 0 {
            if current == Token::Paren(Parens::RightParen) {
                break;
            }
        } else {
            return Err(syntax_error("invalid function definition")?)
        }
    }

    Ok(params)
}

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<FunctionDefinitionNode, ()> {
    // no `fn` keyword
    // example:
    // (param $_) {out param}

    if tokens.len() == 0 {
        return Err(syntax_error("missing function definition")?)
    }

    let first_token =
        tokens.pop_front().unwrap();
    if first_token == Token::Paren(Parens::LeftParen) {
        let function_params =
            params_resolve(tokens)?;

        let next_token =
            tokens.pop_front();
        if next_token != Some(Token::Paren(Parens::LeftBrace)) {
            return Err(syntax_error("missing function body, expected '{'")?)
        }

        let function_body = 
            statement_block::resolve(tokens)?;

        Ok(FunctionDefinitionNode {
            params: function_params,
            name: None,
            body: function_body,
        })
    } else {
        Err(syntax_error("missing function param definition, expected '('")?)
    }
}