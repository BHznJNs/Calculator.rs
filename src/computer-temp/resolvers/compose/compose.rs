use std::rc::Rc;

use crate::computer::resolvers::invocation::invocation_resolve;
use crate::computer::resolvers::variable_reading;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{array_reading, object_reading};

pub fn resolve(
    node:  &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result =
    match node {
        ASTNode::Invocation(sub_node) =>
            invocation_resolve::resolve(sub_node, scope)?,
        ASTNode::ArrayElementReading(sub_node) => {
            let array_value =
                resolve(&sub_node.array_node, scope)?;
            array_reading::resolve(
                array_value,
                &sub_node.index_node,
                scope,
            )?
        },
        ASTNode::ObjectReading(sub_node) => {
            let obj_value =
                resolve(&sub_node.obj_node, scope)?;
            object_reading::resolve(
                obj_value,
                &sub_node.property,
            )?
        },
        ASTNode::Variable(sub_node) =>
            variable_reading::resolve(&sub_node.name, scope)?,
        _ => todo!()
    };
    Ok(result)
}