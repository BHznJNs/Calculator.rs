use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::types::LazyExpressionNode;
use crate::public::error::syntax_error;
use crate::public::value::parens::Parens;

use super::sequence;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<LazyExpressionNode, ()> {
    let first_index = 0;
    let mut sub_tokens = TokenVec::new();
    let mut brace_count = 1;

    while first_index < tokens.len() {
        if first_index == tokens.len() {
            return Err(syntax_error("unmatched brace")?)
        }

        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Parens::LeftBrace) {
            brace_count += 1;
        }
        if current == Token::Paren(Parens::RightBrace) {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }

    let sub_sequence =
        sequence::resolve(&mut sub_tokens)?;

    Ok(LazyExpressionNode { sub_sequence })
}