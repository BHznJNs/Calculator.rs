use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::{ExpressionNode, MapLiteralNode};
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;

use super::expression;

pub fn resolve(tokens: &mut TokenVec) -> Result<MapLiteralNode, ()> {
    let mut brace_count = 1;
    let mut key_stack = Vec::<String>::new();
    let mut expr_stack = Vec::<ExpressionNode>::new();

    while let (Some(token1), Some(token2)) = (tokens.pop_front(), tokens.pop_front()) {
        // pair pattern:
        // String | Identifire : (Expression)
        // example:
        // "key": value
        // key: value

        if let (key, Token::Divider(Divider::Colon)) = (token1, token2) {
            match key {
                Token::Identi(id) => key_stack.push(id),
                Token::String(str) => key_stack.push(str),
                _ => {
                    let msg = format!("invalid map key: {}", key);
                    return Err(syntax_error(&msg)?);
                }
            }

            let mut sub_tokens = TokenVec::new();
            while let Some(token) = tokens.pop_front() {
                if let Token::Paren(paren) = token {
                    match paren {
                        Paren::LeftBrace => brace_count += 1,
                        Paren::RightBrace => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    sub_tokens.push_back(token);
                } else if let Token::Divider(Divider::Semicolon) = token {
                    break;
                } else {
                    sub_tokens.push_back(token);
                }
            }
            let value_expr = expression::resolve(&mut sub_tokens)?;
            expr_stack.push(value_expr);
        } else {
            return Err(syntax_error("invalid map definition")?);
        }
    }

    return Ok(MapLiteralNode {
        keys: key_stack,
        values: expr_stack,
    });
}
