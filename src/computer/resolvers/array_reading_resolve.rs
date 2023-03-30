use std::cell::Ref;
use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ArrayLiteral};

use super::expression_resolve;
use super::variable_reading::variable_reading;

fn index_resolve(
    expression_node: &ASTNode,
    scope: &mut Scope
) -> Result<usize, ()> {
    let index_value =
        expression_resolve::resolve(expression_node, scope)?;
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
        Value::Number(num) =>{
            if is_nested_reading {
                println!("Number has no sub-element.");
                return Err(())
            }
            Ok(Value::Number(*num))
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
       variable_reading(arr_name, scope)?.as_ref() {
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