use std::rc::Rc;

use crate::computer::resolvers::variable_reading;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::error::InternalComponent;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::{computer::resolvers::invocation::invocation_resolve, public::error::internal_error};

use super::{array_reading, object_reading};

pub fn resolve(node: Rc<ASTNode>, scope: &mut Scope) -> Result<Value, ()> {
    let result = match node.as_ref() {
        ASTNode::Invocation(sub_node) => invocation_resolve::resolve(sub_node.clone(), scope)?,
        ASTNode::ArrayElementReading(sub_node) => {
            let array_clone = sub_node.array_node.clone();
            let array_value = resolve(array_clone.into(), scope)?;

            array_reading::resolve(array_value, &sub_node.index_node, scope)?
        }
        ASTNode::ObjectReading(sub_node) => {
            let obj_clone = sub_node.obj_node.clone();
            let obj_value = resolve(obj_clone.into(), scope)?;
            object_reading::resolve(obj_value, &sub_node.property)?
        }
        ASTNode::Variable(sub_node) => variable_reading::resolve(&sub_node.name, scope)?,
        _ => {
            let msg = format!("unexpected ASTNode {} in compose", node);
            return Err(internal_error(InternalComponent::Computer, &msg)?);
        }
    };
    Ok(result)
}
