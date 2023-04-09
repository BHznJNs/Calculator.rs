use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::value::parens::Parens;

use super::sequence;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    let first_index = 0;
    let mut sub_tokens = TokenVec::new();
    let mut brace_count = 1;

    while first_index < tokens.len() {
        if first_index == tokens.len() {
            println!("Unmatched brace.");
            return Err(())
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
    let current_node = ASTNode {
        type__: ASTNodeTypes::LazyExpression,
        params: Some(vec![sub_sequence]),
    };

    Ok(current_node)
}
