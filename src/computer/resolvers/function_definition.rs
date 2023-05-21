use std::rc::Rc;

use crate::public::compile_time::ast::types::FunctionDefinitionNode;
use crate::public::value::function::UserDefinedFunction;

pub fn resolve(node: Rc<FunctionDefinitionNode>) -> Result<UserDefinedFunction, ()> {
    Ok(UserDefinedFunction {
        params: node.params.clone(),
        body: node.body.clone(),
    })
}
