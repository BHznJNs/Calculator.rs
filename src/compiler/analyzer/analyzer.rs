use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};

use crate::compiler::tokenizer::token::TokenVec;
use super::sequence_resolve::sequence_resolve;

pub fn analyze(mut tokens: TokenVec) -> Result<ASTNode, ()> {
    let params =
        vec![sequence_resolve(&mut tokens)?];

    let root = ASTNode {
        type__: ASTNodeTypes::Root,
        params: Some(params),
    };
    Ok(root)
}