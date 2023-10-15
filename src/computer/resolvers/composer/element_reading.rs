use std::cell::RefMut;

use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::{assignment_error, range_error, syntax_error, CalcResult};
use crate::public::run_time::scope::Scope;
use crate::public::value::array::RawArray;
use crate::public::value::map::RawMap;
use crate::public::value::Value;

use super::super::expression;

fn check_outof_range(index: usize, len: usize) -> CalcResult<()> {
    if index >= len {
        Err(range_error(
            "indexing reading",
            format!("index < {}", len),
            index,
        ))
    } else {
        Ok(())
    }
}

fn middle_ware(
    target_value: Value,
    index_value: Value,
    arr_callback: impl Fn(RefMut<RawArray>, usize) -> CalcResult<Value>,
    str_callback: impl Fn(RefMut<String>, usize) -> CalcResult<Value>,
    map_callback: impl Fn(RefMut<RawMap>, &str) -> CalcResult<Value>,
) -> CalcResult<Value> {
    match (&target_value, index_value) {
        (Value::Array(arr), Value::Number(num)) => {
            // array
            let arr_ref = arr.borrow_mut();
            let index = num.int_value() as usize;
            check_outof_range(index, arr_ref.len())?;
            arr_callback(arr_ref, index)
        }
        (Value::String(str), Value::Number(num)) => {
            // string
            let str_ref = str.borrow_mut();
            let index = num.int_value() as usize;
            check_outof_range(index, str_ref.len())?;
            str_callback(str_ref, index)
        }
        (Value::Map(map), Value::String(key)) => {
            // map
            let map_ref = map.borrow_mut();
            let key_temp = key.borrow();
            let key_str = key_temp.as_str();
            map_callback(map_ref, key_str)
        }
        _ => match target_value {
            Value::Array(_) => Err(syntax_error("Array indexing must be Number typed")),
            Value::String(_) => Err(syntax_error("String indexing must be Number typed")),
            Value::Map(_) => Err(syntax_error("Map key must be String typed")),
            _ => Err(syntax_error("invalid indexing")),
        },
    }
}

pub fn resolve(
    target_value: Value,
    index_node: &ExpressionNode,
    scope: &mut Scope,
) -> CalcResult<Value> {
    let index_value = expression::resolve(index_node, scope)?;
    let result = middle_ware(
        target_value,
        index_value,
        |arr_ref, index| Ok(arr_ref[index].clone()),
        |str_ref, index| {
            let ch = str_ref.chars().nth(index).unwrap();
            Ok(String::from(ch).into())
        },
        |map_ref, key| {
            let res = map_ref.get(key);
            match res {
                Some(value) => Ok(value),
                None => Ok(Value::EMPTY),
            }
        },
    )?;
    return Ok(result);
}

pub fn assign(
    target_value: Value, // left-hand value
    index_node: &ExpressionNode,
    value: Value, // right-hand value
    scope: &mut Scope,
) -> CalcResult<()> {
    let index_value = expression::resolve(index_node, scope)?;
    middle_ware(
        target_value,
        index_value,
        |mut arr_ref, index| {
            arr_ref[index] = value.clone();
            Ok(Value::EMPTY)
        },
        |_, _| {
            return Err(assignment_error(
                "Raw-String type does not support element assignment",
            ));
        },
        |mut map_ref, key| {
            map_ref.set(String::from(key), value.clone());
            Ok(Value::EMPTY)
        },
    )?;
    return Ok(());
}
