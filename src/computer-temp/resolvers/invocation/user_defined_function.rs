use crate::computer::resolvers::{sequence, expression};
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::{type_error, syntax_error};
use crate::public::value::function::UserDefinedFunction;
use crate::public::run_time::scope::{Scope, LocalScope};
use crate::public::value::value::Value;

fn call(
    function: &UserDefinedFunction,
    scope: &mut Scope,
) -> Result<Value, ()> {
    for mut node in &function.body {
        let sequence_result =
            sequence::resolve(&mut node, scope)?;
    }
    // let node_count = function.body.len();
    // let mut index = 0;
    // while index < node_count {
    //     let mut node = function.body[index];
    //     let sequence_result =
    //         sequence::resolve(&mut node, scope)?;

    //     // when encount keyword `brk` | `break`,
    //     // function end with current sequence result.
    //     if let Value::Void(Some(val)) = sequence_result {
    //         return Ok(val.unwrap())
    //     }

    //     if index == node_count - 1 {
    //         // when last sequence of function
    //         // return the value of the sequence.
    //         return Ok(sequence_result)
    //     }
    //     index += 1;
    // }
    Ok(Value::Void(None))
}

pub fn invoke(
    function: &UserDefinedFunction,
    params: &Vec<ExpressionNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let mut local_scope = LocalScope::init();
    let mut index = 0;

    if params.len()       < function.params.len() {
//  if actual_param_count < formal_param_count
        let msg = format!(
            "function param missing, expected {}, found {}",
            function.params.len(),
            params.len()
        );
        return Err(syntax_error(&msg)?)
    }
    
    while index < function.params.len() {
        let formal_param =
            &function.params[index];

        let actual_param_node =
            &params[index];
        let actual_param_value =
            expression::resolve(actual_param_node, scope)?;

        // param type check
        if actual_param_value.check_type(formal_param.type__) {
            local_scope.variables.insert(
                formal_param.identi.to_string(),
                actual_param_value
            );
        } else {
            type_error(
                Some(&formal_param.identi),
                formal_param.type__,
                actual_param_value.get_type()
            )?
        }

        index += 1;
    }

    // cached local scope
    let mut local_scope_cached = scope.local.take();

    // assign new scope
    scope.local = Some(local_scope);
    let func_result =
        call(&function, scope)?;

    scope.local = local_scope_cached.take();

    Ok(func_result)
}