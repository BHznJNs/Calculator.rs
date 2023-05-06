use crate::computer::resolvers::{variable_reading, compose::compose};
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::run_time::scope::Scope;
use crate::public::value::function::Function;
use crate::public::value::value::Value;

use super::{build_in_function, lazy_expression, user_defined_function};

fn variable_invoke(
    func_name: &String,
    params: &ASTNodeVec,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let func_value =
        variable_reading::resolve(func_name, scope)?;
    let result =
        func_invoke(func_value, params, scope)?;
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
        Value::Function(func_enum) => {
            match func_enum {
                Function::BuildIn(build_in_fn) => {
                    build_in_function::invoke(
                        build_in_fn.clone(),
                        func_params, scope,
                    )?
                },
                Function::UserDefined(user_defined_fn) => {
                    user_defined_function::invoke(
                        &user_defined_fn, 
                        func_params, scope,
                    )?
                },
            }
        },
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