use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::{ArrayElementReadingNode, ArrayLiteralNode};
use crate::public::compile_time::parens::Paren;

use super::{expression, list};

pub fn literal_resolve(tokens: &mut TokenVec) -> Result<ArrayLiteralNode, ()> {
    let elements = list::resolve(tokens, Paren::RightBracket)?;
    Ok(ArrayLiteralNode { elements })
}

pub fn reading_resolve(
    array_node: ASTNode,
    tokens: &mut TokenVec,
) -> Result<ArrayElementReadingNode, ()> {
    // example:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`

    let first_index = 0;
    let mut bracket_count = 1;
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Paren::LeftBracket) {
            bracket_count += 1;
        }
        if current == Token::Paren(Paren::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }
    let index_node = expression::resolve(&mut sub_tokens)?;
    Ok(ArrayElementReadingNode {
        array_node,
        index_node,
    })
}
