use crate::public::compile_time::ast::types::FunctionDefinitionNode;
use crate::public::error::CalcResult;
use crate::public::value::function::UserDefinedFunction;

pub fn resolve(node: &FunctionDefinitionNode) -> CalcResult<UserDefinedFunction> {
    return Ok(UserDefinedFunction {
        params: node.params.clone(),
        body: node.body.clone(),
    });
}
