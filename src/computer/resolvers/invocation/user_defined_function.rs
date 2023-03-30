use std::rc::Rc;

use crate::computer::resolvers::sequence_resolve;
use crate::public::compile_time::ast::ASTNode;
use crate::public::value::function::UserDefinedFunction;
use crate::public::run_time::scope::{Scope, LocalScope};
use crate::public::value::value::Value;

fn call(
    function: &UserDefinedFunction,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let node_count = function.body.len();
    let mut index = 0;
    while index < node_count {
        let node = &function.body[index];
        let sequence_result =
            sequence_resolve::resolve(node, scope)?;

        // when encount keyword `brk` | `break`,
        // function end with current sequence result.
        if let Value::Void(Some(val)) = sequence_result.as_ref() {
            return Ok(val.clone())
        }

        if index == node_count - 1 {
            // when last sequence of function
            // return the value of the sequence.
            return Ok(sequence_result)
        }
        index += 1;
    }
    Ok(Rc::new(Value::Void(None)))
}

pub fn invoke(
    function: &UserDefinedFunction,
    params: &Vec<ASTNode>,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let mut local_scope = LocalScope::init();
    let mut index = 0;

    while index < function.params.len() {
        if index >= params.len() {
            println!("Function param missing.");
            return Err(())
        }

        let formal_param =
            &function.params[index];

        let actual_param_node = &params[index];
        let actual_param_value =
            sequence_resolve::resolve(actual_param_node, scope)?;

        // param type check
        if actual_param_value.check_type(&formal_param.type__) {
            local_scope.variables.insert(
                formal_param.identi.to_string(),
                actual_param_value
            );
        } else {
            println!("Improper param type, expected {}, found {}.",
                formal_param.type__,
                actual_param_value.get_type()
            );
            return Err(())
        }

        index += 1;
    }

    // cached local scope
    let mut local_scope_cached = if scope.local.is_some() {
        // nested function invocation
        scope.local.take()
    } else {
        None
    };

    // assign new scope
    scope.local = Some(local_scope);
    let func_result =
        call(&function, scope)?;

    scope.local = if local_scope_cached.is_some() {
        local_scope_cached.take()
    } else {
        None
    };

    Ok(func_result)
}