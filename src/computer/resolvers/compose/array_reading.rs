use crate::public::compile_time::ast::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::super::expression;

fn index_resolve(
    expression_node: &ASTNode,
    scope: &mut Scope
) -> Result<usize, ()> {
    let index_value =
        expression::resolve(expression_node, scope)?;
    if let Value::Number(number_box) = index_value {
        let number_value = number_box;
        if number_value < Number::Int(0) {
            println!("Index of an array should not be less than ZERO.");
            return Err(())
        }
        Ok(number_value.int_value() as usize)
    } else {
        println!("Invalid array index.");
        Err(())
    }
}

// --- --- --- --- --- ---

pub fn assign(
    arr_value: Value,
    index_node: &ASTNode,
    value: Value,
    scope: &mut Scope,
) -> Result<(), ()> {
    let Value::Array(arr_ref) =
        arr_value else {
        println!("Invalid array reading.");
        return Err(())
    };
    let index_value =
        index_resolve(index_node, scope)?;
    let mut arr =
        arr_ref.as_ref().borrow_mut();
    arr[index_value] = value;
    Ok(())
}

pub fn resolve(
    arr_rc: Value,
    index_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let Value::Array(arr_ref) =
        arr_rc else {
        println!("Invalid array reading.");
        return Err(())
    };
    let index_value =
        index_resolve(index_node, scope)?;
    let arr = arr_ref.as_ref().borrow();
    Ok(arr[index_value].clone())
}