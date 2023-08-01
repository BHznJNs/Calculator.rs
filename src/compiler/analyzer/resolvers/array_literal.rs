use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::types::ArrayLiteralNode;
use crate::public::compile_time::parens::Paren;

use super::list;

pub fn resolve(tokens: &mut TokenVec) -> Result<ArrayLiteralNode, ()> {
    let elements = list::resolve(tokens, Paren::RightBracket)?;
    return Ok(ArrayLiteralNode { elements });
}
