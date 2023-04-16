use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, Overload};

use super::compose::{array_reading, object_reading, compose};
use super::{expression, array_literal, class, function_definition, instantiation};

fn right_hand_resolve(
    node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match &node.type__ {
        ASTNodeTypes::Expression =>
            expression::resolve(node, scope)?,
        ASTNodeTypes::LazyExpression =>
            Value::create(node.clone()),

        ASTNodeTypes::ArrayLiteral => {
            let array_elements =
                array_literal::resolve(node, scope)?;
            Value::create(array_elements)
        },
        ASTNodeTypes::FunctionDefinition(_) =>
            Value::create(function_definition::resolve(node)?),
        ASTNodeTypes::ClassDefinition => {
            let class_definition =
                class::resolve(&node)?;
            Value::create(class_definition)
        },
        ASTNodeTypes::Instantiation(_) => {
            let inst =
                instantiation::resolve(node, scope)?;
            Value::create(inst)
        },
        _ => {
            println!("Analyzer error for invalid right_hand_node.");
            return Err(())
        }
    };
    Ok(result)
}

pub fn resolve(
    assignment_node: &ASTNode,
    scope: &mut Scope
) -> Result<Value, ()> {
    let ASTNodeTypes::Assignment(left_hand_rc) =
        &assignment_node.type__ else {
        println!("Invalid variable name for assignment.");
        return Err(())
    };
    let right_hand_node = &assignment_node
        .params
        .as_ref()
        .unwrap()[0];

    let right_hand_value =
        right_hand_resolve(right_hand_node, scope)?;
    
    let params =
        left_hand_rc
        .params
        .as_ref();

    match &left_hand_rc.type__ {
        ASTNodeTypes::Variable(var_name) => {
            // if local-scope, assign variable to
            // the local-scope is preferred.
            match &mut scope.local {
                Some(local_scope) =>
                    // usually in a function invocation.
                    local_scope.variables.insert(
                        var_name.to_owned(),
                        right_hand_value.clone()
                    ),
                None =>
                    scope.global.variables.insert(
                        var_name.to_owned(),
                        right_hand_value.clone()
                    ),
            };
        },
        ASTNodeTypes::ArrayElementReading(sub_node) => {
            let left_hand_arr =
                compose::resolve(sub_node, scope)?;
            array_reading::assign(
                left_hand_arr,
                &params.unwrap()[0],
                right_hand_value.clone(),
                scope
            )?;
        },
        ASTNodeTypes::ObjectReading(sub_node) => {
            let left_hand_obj =
                compose::resolve(sub_node, scope)?;
            object_reading::assign(
                left_hand_obj,
                &params.unwrap()[0],
                right_hand_value.clone(),
            )?;
        },
        _ => {
            println!("Invalid left-hand value.");
            return Err(())
        }
    }

    Ok(right_hand_value)
}