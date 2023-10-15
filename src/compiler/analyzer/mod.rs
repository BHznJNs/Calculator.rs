mod resolvers;

use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::ast_enum::RootNode;
use crate::public::error::CalcResult;

use resolvers::sequence;

pub fn analyze(mut tokens: TokenVec) -> CalcResult<RootNode> {
    let sub_node = sequence::resolve(&mut tokens)?;
    let root = RootNode { sub_node };
    return Ok(root);
}
