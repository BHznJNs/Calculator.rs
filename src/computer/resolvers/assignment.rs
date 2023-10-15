use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::AssignmentNode;
use crate::public::error::{assignment_error, CalcResult};
use crate::public::run_time::scope::Scope;
use crate::public::value::Value;

use super::composer::{compose, element_reading, object_reading};
use super::expression;

pub fn resolve(node: &AssignmentNode, scope: &mut Scope, is_global: bool) -> CalcResult<Value> {
    let left_hand_node = &node.left_hand_node;
    let right_hand_node = &node.right_hand_node;
    let right_hand_value = expression::resolve(right_hand_node, scope)?;

    match left_hand_node {
        ASTNode::Variable(sub_node) => {
            if is_global {
                scope
                    .global
                    .variables
                    .insert(sub_node.name.clone(), right_hand_value.clone());
            } else {
                scope.assign(sub_node.name.clone(), right_hand_value.clone());
            }
        }

        ASTNode::ElementReading(sub_node) => {
            let target_node = &sub_node.target_node;
            let target_value = compose::resolve(target_node, scope)?;
            element_reading::assign(
                target_value,
                &sub_node.index_node,
                right_hand_value.clone(),
                scope,
            )?;
        }
        ASTNode::ObjectReading(sub_node) => {
            let sub_obj_node = &sub_node.obj_node;
            let obj_value = compose::resolve(sub_obj_node, scope)?;
            object_reading::assign(obj_value, &sub_node.property, right_hand_value.clone())?;
        }
        _ => return Err(assignment_error("invalid left-hand value")),
    }
    return Ok(right_hand_value);
}
