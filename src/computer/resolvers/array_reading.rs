use std::cell::{Ref, RefMut};
use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ArrayLiteral};

use super::expression;
use super::variable_reading;

fn index_resolve(
    expression_node: &ASTNode,
    scope: &mut Scope
) -> Result<usize, ()> {
    let index_value =
        expression::resolve(expression_node, scope)?;
    if let Value::Number(number_box) = index_value.as_ref() {
        let number_value = *number_box;
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

fn write(
    mut arr_ref: RefMut<ArrayLiteral>,
    params: &Vec<ASTNode>,
    value: Rc<Value>,
    scope: &mut Scope,
) -> Result<(), ()> {
    let index =
        index_resolve(&params[0], scope)?;

    if index >= arr_ref.len() {
        println!("Index out of range.");
        return Err(())
    }
    let is_nested_reading = params.len() > 1;

    match &arr_ref[index] {
        Value::Number(_)   |
        Value::String(_)   |
        Value::Function(_) |
        Value::Object(_) => {
            if is_nested_reading {
                println!("Target element has no sub-element.");
                return Err(())
            }
            arr_ref[index] = value.unwrap();
            Ok(())
        },
        Value::Array(arr) => {
            if is_nested_reading {
                let cloned = arr.clone();
                let arr_ref = cloned.as_ref().borrow_mut();
                let sub_params = params[1]
                    .params
                    .as_ref()
                    .unwrap();
                Ok(write(arr_ref, sub_params, value, scope)?)
            } else {
                arr_ref[index] = value.unwrap();
                Ok(())
            }
        },
        _ => {
            println!("Unexpected element in array.");
            return Err(())
        }
    }
}

pub fn assign(
    node: &ASTNode,
    value: Rc<Value>,
    scope: &mut Scope,
) -> Result<(), ()> {
    let ASTNodeTypes::ArrayElementReading(arr_name) =
        &node.type__ else {
        println!("Unexpected error.");
        return Err(())
    };
    let params = node
        .params
        .as_ref()
        .unwrap();

    if let Value::Array(arr) =
       variable_reading::resolve(arr_name, scope)?.as_ref() {
        let cloned = arr.clone();
        let arr_ref =
            cloned.as_ref().borrow_mut();
        write(arr_ref, params, value, scope)?;
        Ok(())
    } else {
        println!("'{}' is not an valid array.", arr_name);
        return Err(())
    }
}

// --- --- --- --- --- ---

fn read(
    arr_ref: Ref<ArrayLiteral>,
    params: &Vec<ASTNode>,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let index =
        index_resolve(&params[0], scope)?;

    if index >= arr_ref.len() {
        println!("Index out of range.");
        return Err(())
    }
    let is_nested_reading = params.len() > 1;

    match &arr_ref[index] {
        Value::Number(_)   |
        Value::String(_)   |
        Value::Function(_) |
        Value::Object(_) => {
            if is_nested_reading {
                println!("Target element has no sub-element.");
                return Err(())
            }
            Ok(arr_ref[index].clone())
        },
        Value::Array(arr) => {
            if is_nested_reading {
                let cloned = arr.clone();
                let arr_ref = cloned.as_ref().borrow();
                let sub_params = params[1]
                    .params
                    .as_ref()
                    .unwrap();
                Ok(read(arr_ref, sub_params, scope)?)
            } else {
                // wrap ArrayLiteral with Value
                Ok(Value::Array(arr.clone()))
            }
        },
        _ => {
            println!("Unexpected element in array.");
            return Err(())
        }
    }
}

pub fn resolve(
    node: &ASTNode,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let ASTNodeTypes::ArrayElementReading(arr_name) =
        &node.type__ else {
        println!("Unexpected error.");
        return Err(())
    };
    let params = node
        .params
        .as_ref()
        .unwrap();


    if let Value::Array(arr) =
       variable_reading::resolve(arr_name, scope)?.as_ref() {
        let cloned = arr.clone();
        let arr_ref = cloned.as_ref().borrow();
        let result =
            read(arr_ref, params, scope)?;
        Ok(Rc::new(result))
    } else {
        println!("'{}' is not an valid array.", arr_name);
        return Err(())
    }
}