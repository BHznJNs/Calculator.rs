use crate::computer::resolvers::sequence;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::error::CalcResult;
use crate::public::run_time::scope::Scope;
use crate::public::value::Value;

#[inline]
pub fn invoke(le_body: &ASTNode, scope: &mut Scope) -> CalcResult<Value> {
    // le -> lazy_expression
    sequence::resolve(le_body, scope)
}
