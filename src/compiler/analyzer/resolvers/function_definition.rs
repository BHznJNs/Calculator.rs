use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::FunctionDefinitionNode;
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;
use crate::public::value::function::UserDefinedFnParam;
use crate::public::value::ValueType;

use super::statement_block;

// refactor: params_resolve
fn params_resolve(tokens: &mut TokenVec) -> Result<Vec<UserDefinedFnParam>, ()> {
    // structure:
    // identi annotation) {function body ...}

    let mut params = vec![];

    while let Some(current) = tokens.pop_front() {
        match current {
            Token::Identi(identi) => {
                let Some(next) = tokens.pop_front() else {
                    return Err(syntax_error(
                        "incompleted function definition",
                    )?);
                };
                if let Token::Annotation(type__) = next {
                    params.push(UserDefinedFnParam { type__, identi })
                } else if let Token::Divider(Divider::Comma) | Token::Paren(Paren::RightParen) =
                    next
                {
                    tokens.push_front(next);
                    params.push(UserDefinedFnParam {
                        type__: ValueType::Void,
                        identi,
                    });
                } else {
                    return Err(syntax_error(
                        "type annotation expected in function definition",
                    )?);
                }
            }
            Token::Divider(Divider::Comma) => continue,
            Token::Paren(Paren::RightParen) => break,
            _ => {
                let msg = format!("unexpected token {} in function param", current);
                return Err(syntax_error(&msg)?);
            }
        }
    }
    return Ok(params);
}

pub fn resolve(tokens: &mut TokenVec) -> Result<FunctionDefinitionNode, ()> {
    // no `fn` keyword
    // example:
    // (param $_) {out param}

    if tokens.is_empty() {
        return Err(syntax_error("missing function definition")?);
    }

    let first_token = tokens.pop_front().unwrap();
    if first_token == Token::Paren(Paren::LeftParen) {
        let function_params = params_resolve(tokens)?;

        let next_token = tokens.pop_front();
        if next_token != Some(Token::Paren(Paren::LeftBrace)) {
            return Err(syntax_error("missing function body, expected '{'")?);
        }

        let function_body = statement_block::resolve(tokens)?;

        Ok(FunctionDefinitionNode {
            params: function_params,
            name: None,
            body: function_body,
        })
    } else {
        Err(syntax_error(
            "missing function param definition, expected '('",
        )?)
    }
}
