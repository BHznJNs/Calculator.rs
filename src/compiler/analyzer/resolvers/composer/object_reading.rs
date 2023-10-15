use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::ObjectReadingNode;
use crate::public::error::{syntax_error, CalcResult};

pub fn resolve(obj_node: ASTNode, tokens: &mut TokenVec) -> CalcResult<ObjectReadingNode> {
    // object property / method reading
    let Some(Token::Identi(property)) = tokens.pop_front() else {
        return Err(syntax_error("missing object property"));
    };
    return Ok(ObjectReadingNode { obj_node, property });
}
