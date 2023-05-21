use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::ObjectReadingNode;
use crate::public::error::syntax_error;

pub fn resolve(
    obj_node: ASTNode,
    tokens: &mut TokenVec
) -> Result<ObjectReadingNode, ()> {
    // object property / method reading

    let Some(Token::Identi(property)) =
        tokens.pop_front() else {
        return Err(syntax_error("missing object property")?)
    };

    Ok(ObjectReadingNode {
        obj_node,
        property,
    })
}