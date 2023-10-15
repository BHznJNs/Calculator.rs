use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::types::ArrayLiteralNode;
use crate::public::compile_time::parens::Paren;
use crate::public::error::CalcResult;

use super::list;

pub fn resolve(tokens: &mut TokenVec) -> CalcResult<ArrayLiteralNode> {
    let elements = list::resolve(tokens, Paren::RightBracket)?;
    return Ok(ArrayLiteralNode { elements });
}
