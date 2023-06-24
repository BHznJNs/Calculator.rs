use std::borrow::Borrow;

use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::error::InternalComponent;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::{computer::resolvers::invocation::invocation_resolve, public::error::internal_error};

use super::{array_reading, object_reading};

pub fn resolve(node: &ASTNode, scope: &mut Scope) -> Result<Value, ()> {
    let result = match node {
        ASTNode::Invocation(sub_node) => invocation_resolve::resolve(sub_node.borrow(), scope)?,
        ASTNode::ArrayElementReading(sub_node) => {
            let sub_array_node = &sub_node.array_node;
            let array_value = resolve(sub_array_node, scope)?;

            array_reading::resolve(array_value, &sub_node.index_node, scope)?
        }
        ASTNode::ObjectReading(sub_node) => {
            let sub_obj_node = &sub_node.obj_node;
            let obj_value = resolve(sub_obj_node, scope)?;
            object_reading::resolve(obj_value, &sub_node.property)?
        }
        ASTNode::Variable(sub_node) => scope.read_var(&sub_node.name)?,
        _ => {
            let msg = format!("unexpected ASTNode {} in compose", node);
            return Err(internal_error(InternalComponent::Computer, &msg)?);
        }
    };
    Ok(result)
}
