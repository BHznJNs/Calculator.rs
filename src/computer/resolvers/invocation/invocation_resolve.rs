use std::rc::Rc;

use crate::computer::resolvers::variable_reading;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::{build_in_function, lazy_expression, user_defined_function};

pub fn resolve(
    invocation_node: &ASTNode,
    scope: &mut Scope
) -> Result<Rc<Value>, ()> {
    let ASTNodeTypes::Invocation(func_name) = &invocation_node.type__ else {
        println!("Unexpected error: invalid function name.");
        return Err(())
    };
    let params = invocation_node
        .params
        .as_ref()
        .unwrap();

    let result =
    match scope.global.build_in_funcs.get(func_name.as_str()) {
        Some(func_struct) => {
            build_in_function::invoke(
                func_struct.clone(),
                params, scope
            )?
        },
        None => {
            let func =
                variable_reading::resolve(func_name, scope)?;
            match func.as_ref() {
                Value::LazyExpression(le) =>
                    lazy_expression::invoke(le, scope)?,
                Value::Function(func_struct) =>
                    user_defined_function::invoke(
                        func_struct, 
                        params, scope
                    )?,
                _ => {
                    println!("'{}' is not a valid callable target.", func_name);
                    return Err(())
                }
            }
        }
    };

    Ok(result)
}