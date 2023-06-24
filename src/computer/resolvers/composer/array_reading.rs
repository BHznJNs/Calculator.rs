use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::{range_error, type_error};
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Overload, Value, ValueType};

use super::super::expression;

fn index_resolve(expression_node: &ExpressionNode, scope: &mut Scope) -> Result<usize, ()> {
    let index_value = expression::resolve(expression_node, scope)?;
    if let Value::Number(num) = index_value {
        if num < Number::Int(0) {
            return Err(range_error(
                "array reading",
                "> 0",
                num.int_value() as usize,
            )?);
        }
        Ok(num.int_value() as usize)
    } else {
        // index type error
        Err(type_error(
            Some("array index"),
            vec![ValueType::Number],
            index_value.get_type(),
        )?)
    }
}

fn check_outof_range(index: usize, len: usize) -> Result<(), ()> {
    if index >= len {
        Err(range_error(
            "indexing reading",
            format!("index < {}", len),
            index,
        )?)
    } else {
        Ok(())
    }
}

// --- --- --- --- --- ---

pub fn assign(
    array_value: Value,
    index_node: &ExpressionNode,
    value: Value,
    scope: &mut Scope,
) -> Result<(), ()> {
    let index_value = index_resolve(index_node, scope)?;
    if let Value::Array(arr_ref) = array_value {
        // array writing
        let mut arr = arr_ref.as_ref().borrow_mut();
        check_outof_range(index_value, arr.len())?;
        arr[index_value] = value;
    } else if let Value::String(str_ref) = array_value {
        // string writing
        let mut str = str_ref.as_ref().borrow_mut();

        let Value::String(target) = value else {
            return Err(type_error(
                Some("string assignment"),
                vec![ValueType::String],
                value.get_type(),
            )?)
        };
        check_outof_range(index_value, str.len())?;
        let char_str = &target.as_ref().borrow();
        str.replace_range(index_value..index_value + 1, char_str);
    } else {
        return Err(type_error(
            Some("indexing assignment"),
            vec![ValueType::String, ValueType::Array],
            array_value.get_type(),
        )?);
    }
    Ok(())
}

pub fn resolve(
    array_value: Value,
    index_node: &ExpressionNode,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let index_value = index_resolve(index_node, scope)?;

    if let Value::Array(arr_ref) = array_value {
        let arr = arr_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, arr.len())?;
        Ok(arr[index_value].clone())
    } else if let Value::String(str_ref) = array_value {
        let str = str_ref.as_ref().borrow();
        // check if out of range
        check_outof_range(index_value, str.len())?;
        let slice = &str[index_value..index_value + 1];
        Ok(Value::create(slice.to_string()))
    } else {
        Err(type_error(
            Some("indexing"),
            vec![ValueType::String, ValueType::Array],
            array_value.get_type(),
        )?)
    }
}
