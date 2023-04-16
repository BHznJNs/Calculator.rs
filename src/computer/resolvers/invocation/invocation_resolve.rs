use crate::computer::resolvers::{variable_reading, compose::compose};
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{build_in_function, lazy_expression, user_defined_function};

fn variable_invoke(
    func_name: &String,
    params: &ASTNodeVec,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result =
    match scope.global.build_in_funcs.get(func_name.as_str()) {
        Some(func_struct) => {
            // invoke build-in function has
            // the higher priority.
            build_in_function::invoke(
                func_struct.clone(),
                params, scope
            )?
        },
        None => {
            let func =
                variable_reading::resolve(func_name, scope)?;
            func_invoke(func, params, scope)?
        }
    };
    Ok(result)
}

fn func_invoke(
    func_value: Value,
    func_params: &ASTNodeVec,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let func_result =
    match func_value {
        Value::LazyExpression(le) =>
            lazy_expression::invoke(&le, scope)?,
        Value::Function(func_struct) =>
            user_defined_function::invoke(
                &func_struct, 
                func_params, scope
            )?,
        _ => {
            println!("Invalid callable target.");
            return Err(())
        }
    };
    Ok(func_result)
}

pub fn resolve(
    node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let ASTNodeTypes::Invocation(func_node) =
        &node.type__ else {
        println!("Unexpected error: invalid function name.");
        return Err(())
    };

    let params = node
        .params
        .as_ref()
        .unwrap();

    let func_result =
    match &func_node.type__ {
        ASTNodeTypes::Variable(func_name) => {
            variable_invoke(func_name, params, scope)?
        },
        ASTNodeTypes::Invocation(_) |
        ASTNodeTypes::ObjectReading(_) |
        ASTNodeTypes::ArrayElementReading(_) => {
            let func_value =
                compose::resolve(func_node, scope)?;
            func_invoke(func_value, params, scope)?
        },
        _ => {
            println!("Invalid callable target.");
            return Err(())
        }
    };
    Ok(func_result)
}