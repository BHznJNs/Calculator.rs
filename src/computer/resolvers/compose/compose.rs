use crate::computer::resolvers::invocation::invocation_resolve;
use crate::computer::resolvers::variable_reading;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{array_reading, object_reading};

pub fn resolve(
    node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let params =
        node.params
        .as_ref();

    let result =
    match &node.type__ {
        ASTNodeTypes::Invocation(_) =>
            invocation_resolve::resolve(node, scope)?,
        ASTNodeTypes::ArrayElementReading(sub_node) => {
            let arr_value = resolve(sub_node, scope)?;
            array_reading::resolve(
                arr_value,
                &params.unwrap()[0],
                scope
            )?
        },
        ASTNodeTypes::ObjectReading(sub_node) => {
            let obj_node = resolve(sub_node, scope)?;
            object_reading::resolve(
                obj_node,
                &params.unwrap()[0],
            )?
        },
        ASTNodeTypes::Variable(var_name) =>
            variable_reading::resolve(var_name, scope)?,
        _ => todo!()
    };
    Ok(result)
}