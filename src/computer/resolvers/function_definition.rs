use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::value::function::UserDefinedFunction;

pub fn resolve(
    node: &ASTNode
) -> Result<UserDefinedFunction, ()> {
    let ASTNodeTypes::FunctionDefinition(func_params)
        = &node.type__ else {
        println!("Invalid function definition.");
        return Err(())
    };

    let params =
        node.params
        .as_ref()
        .unwrap()
        .to_owned();

    Ok(UserDefinedFunction {
        params: func_params.to_owned(),
        body: params
    })
}