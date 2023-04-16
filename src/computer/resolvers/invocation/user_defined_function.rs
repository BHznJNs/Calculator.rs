use crate::computer::resolvers::sequence;
use crate::public::compile_time::ast::ASTNode;
use crate::public::value::function::UserDefinedFunction;
use crate::public::run_time::scope::{Scope, LocalScope};
use crate::public::value::value::Value;

fn call(
    function: &UserDefinedFunction,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let node_count = function.body.len();
    let mut index = 0;
    while index < node_count {
        let node = &function.body[index];
        let sequence_result =
            sequence::resolve(node, scope)?;

        // when encount keyword `brk` | `break`,
        // function end with current sequence result.
        if let Value::Void(Some(val)) = sequence_result {
            return Ok(val.unwrap())
        }

        if index == node_count - 1 {
            // when last sequence of function
            // return the value of the sequence.
            return Ok(sequence_result)
        }
        index += 1;
    }
    Ok(Value::Void(None))
}

pub fn invoke(
    function: &UserDefinedFunction,
    params: &Vec<ASTNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let mut local_scope = LocalScope::init();
    let mut index = 0;

    while index < function.params.len() {
        if index >= params.len() {
            println!("User defined function param missing.");
            return Err(())
        }

        let formal_param =
            &function.params[index];

        let actual_param_node = &params[index];
        let actual_param_value =
            sequence::resolve(actual_param_node, scope)?;

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
    let mut local_scope_cached = scope.local.take();

    // assign new scope
    scope.local = Some(local_scope);
    let func_result =
        call(&function, scope)?;

    scope.local = local_scope_cached.take();

    Ok(func_result)
}