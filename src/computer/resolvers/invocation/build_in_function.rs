use crate::computer::resolvers::expression;
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::{LocalScope, Scope};
use crate::public::std::modules::BuildInFnCall;
use crate::public::value::function::{BuildInFunction, Function};
use crate::public::value::value::Value;

fn call(function: &BuildInFunction, scope: &mut Scope) -> Result<Value, ()> {
    match &function.identi {
        BuildInFnIdenti::Basic(basic_fn) => basic_fn.call(scope),
        BuildInFnIdenti::Math(math_fn) => math_fn.call(scope),
        BuildInFnIdenti::Array(arr_fn) => arr_fn.call(scope),
        BuildInFnIdenti::String(str_fn) => str_fn.call(scope),
        BuildInFnIdenti::FileSystem(fs_fn) => fs_fn.call(scope),
    }
}

pub fn invoke(
    function: &BuildInFunction,
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

    // cache local scope
    let mut local_scope_cached = scope.local.take();

    scope.local = Some(local_scope);
    let fn_result = call(&function, scope)?;
    scope.local = None;

    scope.local = local_scope_cached.take();

    Ok(fn_result)
}
