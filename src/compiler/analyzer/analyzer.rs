use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::ast_enum::RootNode;

use super::resolvers::sequence;

pub fn analyze(mut tokens: TokenVec) -> Result<RootNode, ()> {
    let sub_node = sequence::resolve(&mut tokens)?;

    let root = RootNode { sub_node };
    Ok(root)
}
