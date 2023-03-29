use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::function::UserDefinedFunction;
use crate::public::value::value::{Value, Overload};

use super::{expression_resolve, array_resolve};

pub fn resolve(
    assignment_node: &ASTNode,
    scope: &mut Scope
) -> Result<Rc<Value>, ()> {
    let ASTNodeTypes::Assignment(var_name) = &assignment_node.type__ else {
        println!("Invalid variable name for assignment.");
        return Err(())
    };
    let right_hand_node = &assignment_node
        .params
        .as_ref()
        .unwrap()[0];

    let right_hand_value = match &right_hand_node.type__ {
        ASTNodeTypes::Expression =>
            expression_resolve::resolve(&right_hand_node, scope)?,
        ASTNodeTypes::LazyExpression =>
            Rc::new(Value::create(right_hand_node.clone())),
        ASTNodeTypes::FunctionDefinition(func_params) => {
            Rc::new(Value::create(UserDefinedFunction {
                params: func_params.to_owned(),
                body: right_hand_node.params
                      .as_ref().unwrap().to_owned(),
            }))
        },
        ASTNodeTypes::ArrayLiteral => {
            let array_elements =
                array_resolve::resolve(&right_hand_node, scope)?;
            Rc::new(Value::create(array_elements))
        },
        _ => {
            println!("Analyzer error for invalid right_hand_node.");
            return Err(())
        }
    };

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

    if let Value::LazyExpression(_) | Value::Function(_) = *right_hand_value {
        Ok(Value::empty(None))
    } else {
        Ok(right_hand_value)
    }
}