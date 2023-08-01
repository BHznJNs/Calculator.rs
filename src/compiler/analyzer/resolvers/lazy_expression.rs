use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::LazyExpressionNode;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;

use super::sequence;

pub fn resolve(tokens: &mut TokenVec) -> Result<LazyExpressionNode, ()> {
    let mut sub_tokens = TokenVec::new();
    let mut brace_count = 1;

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBrace) {
            brace_count += 1;
        }
        if token == Token::Paren(Paren::RightBrace) {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(token);
    }
    if brace_count > 0 {
        return Err(syntax_error("unmatched brace")?);
    }

    let sub_sequence = sequence::resolve(&mut sub_tokens)?;
    return Ok(LazyExpressionNode { sub_sequence });
}
