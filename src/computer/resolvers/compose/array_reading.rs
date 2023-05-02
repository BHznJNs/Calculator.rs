use crate::public::compile_time::ast::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, Overload};

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

fn check_outof_range(
    index: usize,
    len: usize,
) -> Result<(), ()> {
    if index >= len {
        println!("Array reading out of range, expected index < {}, found {}.", len, index);
        Err(())
    } else {
        Ok(())
    }
}

// --- --- --- --- --- ---

pub fn assign(
    arr_value: Value,
    index_node: &ASTNode,
    value: Value,
    scope: &mut Scope,
) -> Result<(), ()> {
    let index_value =
        index_resolve(index_node, scope)?;
    if let Value::Array(arr_ref) = arr_value {
        let mut arr =
            arr_ref.as_ref().borrow_mut();
        check_outof_range(index_value, arr.len())?;
        arr[index_value] = value;
    } else
    if let Value::String(str_ref) = arr_value {
        let mut str =
            str_ref.as_ref().borrow_mut();
        let Value::String(target) = value else {
            println!("Invalid element for String.");
            return Err(())
        };
        check_outof_range(index_value, str.len())?;
        let char_str =
            &target.as_ref().borrow();
        str.replace_range(index_value..index_value+1, char_str);
    } else {
        println!("Invalid array reading.");
        return Err(())
    }
    Ok(())
}

pub fn resolve(
    arr_rc: Value,
    index_node: &ASTNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let index_value =
        index_resolve(index_node, scope)?;

    if let Value::Array(arr_ref) = arr_rc {
        let arr =
            arr_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, arr.len())?;
        Ok(arr[index_value].clone())
    } else
    if let Value::String(str_ref) = arr_rc {
        let str =
            str_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, str.len())?;
        let slice = &str[index_value..index_value+1];
        Ok(Value::create(slice.to_string()))
    } else {
        println!("Invalid array reading.");
        Err(())
    }
}