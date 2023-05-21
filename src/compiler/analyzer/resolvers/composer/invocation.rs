use crate::compiler::analyzer::resolvers::list;
use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::InvocationNode;
use crate::public::compile_time::parens::Paren;

pub fn resolve(caller: ASTNode, tokens: &mut TokenVec) -> Result<InvocationNode, ()> {
    let params = list::resolve(tokens, Paren::RightParen)?;
    Ok(InvocationNode { caller, params })
}
