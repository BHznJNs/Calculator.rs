use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::{ast_enum::ASTNode, types::ElementReadingNode};
use crate::public::compile_time::parens::Paren;

use super::super::expression;

pub fn resolve(target_node: ASTNode, tokens: &mut TokenVec) -> Result<ElementReadingNode, ()> {
    // example for ArrayReading:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`
    // example for MapReading:
    // "prop"] from `map["prop"]`

    let mut bracket_count = 1;
    let mut sub_tokens = TokenVec::new();

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBracket) {
            bracket_count += 1;
        }
        if token == Token::Paren(Paren::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(token);
    }
    let index_node = expression::resolve(&mut sub_tokens)?;
    return Ok(ElementReadingNode {
        target_node,
        index_node,
    });
}
