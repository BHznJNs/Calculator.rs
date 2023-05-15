use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::AssignmentNode;
use crate::public::error::assignment_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::compose::{array_reading, object_reading, compose};
use super::expression;

// fn right_hand_resolve(
//     node: &ASTNode,
//     scope: &mut Scope,
// ) -> Result<Value, ()> {
//     let result = match &node.type__ {
//         ASTNodeTypes::Expression =>
//             expression::resolve(node, scope)?,
//         ASTNodeTypes::LazyExpression =>
//             Value::create(node.clone()),

//         ASTNodeTypes::ArrayLiteral => {
//             let array_elements =
//                 array_literal::resolve(node, scope)?;
//             Value::create(array_elements)
//         },
//         ASTNodeTypes::FunctionDefinition(_) =>
//             Value::create(function_definition::resolve(node)?),
//         ASTNodeTypes::ClassDefinition => {
//             let class_definition =
//                 class::resolve(&node)?;
//             Value::create(class_definition)
//         },
//         ASTNodeTypes::Instantiation(_) => {
//             let inst =
//                 instantiation::resolve(node, scope)?;
//             Value::create(inst)
//         },
//         _ => {
//             return Err(assignment_error("invalid right-hand value")?)
//         }
//     };
//     Ok(result)
// }

pub fn resolve(
    node:  &AssignmentNode,
    scope: &mut Scope
) -> Result<Value, ()> {
    let left_hand_node =
        &node.left_hand_node;
    let right_hand_value =
        expression::resolve(&node.right_hand_node, scope)?;

    match left_hand_node.as_ref() {
        ASTNode::Variable(sub_node) => {
            // if local-scope, assign variable to
            // the local-scope is preferred.
            match &mut scope.local {
                Some(local_scope) =>
                    // usually in a function invocation.
                    local_scope.variables.insert(
                        sub_node.name.to_owned(),
                        right_hand_value.clone()
                    ),
                None =>
                    scope.global.variables.insert(
                        sub_node.name.to_owned(),
                        right_hand_value.clone()
                    ),
            };
        },
        ASTNode::ArrayElementReading(sub_node) => {
            let array_value =
                compose::resolve(&sub_node.array_node, scope)?;
            array_reading::assign(
                array_value,
                &sub_node.index_node,
                right_hand_value.clone(),
                scope,
            )?;
        },
        ASTNode::ObjectReading(sub_node) => {
            let obj_value =
                compose::resolve(&sub_node.obj_node, scope)?;
            object_reading::assign(
                obj_value,
                &sub_node.property,
                right_hand_value.clone(),
            )?;
        },
        _ => return Err(assignment_error("invalid left-hand value")?)
    }

    Ok(right_hand_value)
}