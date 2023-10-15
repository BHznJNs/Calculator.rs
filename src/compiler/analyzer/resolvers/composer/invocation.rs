use crate::compiler::analyzer::resolvers::list;
use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::InvocationNode;
use crate::public::compile_time::parens::Paren;
use crate::public::error::CalcResult;

pub fn resolve(caller: ASTNode, tokens: &mut TokenVec) -> CalcResult<InvocationNode> {
    let params = list::resolve(tokens, Paren::RightParen)?;
    return Ok(InvocationNode { caller, params });
}
