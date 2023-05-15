use std::rc::Rc;

use crate::computer::resolvers::expression;
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::type_error;
use crate::public::std::modules::{math, array, basic, string};
use crate::public::std::std::StdModules;
use crate::public::value::function::BuildInFunction;
use crate::public::run_time::scope::{Scope, LocalScope};
use crate::public::value::value::Value;

fn call(
    function: &BuildInFunction,
    scope: &mut Scope,
) -> Result<Value, ()> {
    match function.lib {
        StdModules::Basic =>
            basic::implement(&function.body, scope),
        StdModules::Math =>
            math::implement(&function.body, scope),
        StdModules::Array =>
            array::implement(&function.body, scope),
        StdModules::String =>
            string::implement(&function.body, scope),
        StdModules::FileSystem =>
            todo!(),
    }
}

pub fn invoke(
    function: Rc<BuildInFunction>,
    params: &Vec<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let mut local_scope = LocalScope::init();
    let mut index = 0;

    while index < function.params.len() {
        let formal_param =
            &function.params[index];

        match formal_param {
            Some(p) => {
                if index >= params.len() {
                    println!("Build-in function param missing.");
                    return Err(())
                }

                let actual_param_node =
                    &params[index];
                let actual_param_value =
                    expression::resolve(actual_param_node, scope)?;

                // param type check
                if actual_param_value.check_type(p.type__) {
                    local_scope.variables.insert(
                        p.identi.to_string(),
                        actual_param_value
                    );
                } else {
                    type_error(
                        Some(p.identi),
                        p.type__,
                        actual_param_value.get_type()
                    )?
                }
            },
            None => break,
        }
        index += 1;
    }

    // cached local scope
    let mut local_scope_cached = scope.local.take();

    scope.local = Some(local_scope);
    let func_result =
        call(&function, scope)?;
    scope.local = None;

    scope.local = local_scope_cached.take();

    Ok(func_result)
}