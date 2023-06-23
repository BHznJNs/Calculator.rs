use crate::computer::resolvers::{expression, sequence};
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::run_time::scope::{LocalScope, Scope};
use crate::public::value::function::{Function, UserDefinedFunction};
use crate::public::value::value::{Value, VoidSign};

fn call(function: &UserDefinedFunction, scope: &mut Scope) -> Result<Value, ()> {
    for node in &function.body {
        let sequence_result = sequence::resolve(node, scope)?;

        if let Value::Void(VoidSign::Break(val)) = sequence_result {
            return Ok(val.unwrap());
        }
    }

    Ok(Value::Void(VoidSign::Empty))
}

pub fn invoke(
    function: &UserDefinedFunction,
    params: &Vec<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let mut local_scope = LocalScope::init();

    Function::param_check(
        &function.params,
        params,
        scope,
        &mut local_scope,
        expression::resolve,
    )?;

    // cached local scope
    let mut local_scope_cached = scope.local.take();

    // assign new scope
    scope.local = Some(local_scope);
    let fn_result = call(&function, scope)?;

    scope.local = local_scope_cached.take();

    Ok(fn_result)
}
