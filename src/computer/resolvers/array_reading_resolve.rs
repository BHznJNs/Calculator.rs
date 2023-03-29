use std::cell::Ref;
use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ArrayLiteral};

use super::expression_resolve;

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

    match scope.global.variables.get(arr_name) {
        Some(var_value) => {
            match var_value.as_ref() {
                Value::Array(arr) => {
                    let cloned = arr.clone();
                    let arr_ref = cloned.as_ref().borrow();
                    let result =
                        read(arr_ref, params, scope)?;
                    Ok(Rc::new(result))
                },
                _ => {
                    println!("'{}' is not an valid array.", arr_name);
                    return Err(())
                }
            }
        },
        None => {
            println!("Variable '{}' is undefined.", arr_name);
            return Err(())
        },
    }
}

// pub fn __resolve(
//     node: &ASTNode,
//     scope: &mut Scope,
// ) -> Result<Rc<Value>, ()> {
//     let ASTNodeTypes::ArrayElementReading(arr_name) =
//         &node.type__ else {
//         println!("Unexpected error.");
//         return Err(())
//     };
//     let mut params = node
//         .params
//         .as_ref()
//         .unwrap();
//     // let mut arr_rc: Rc<ArrayLiteral>;
//     // let mut arr_rc: Rc<RefCell<ArrayLiteral>>;
//     let mut arr_cloned: Rc<RefCell<ArrayLiteral>>;
//     let mut arr_rc: Ref<ArrayLiteral>;

//     let result =
//     loop {
//         match scope.global.variables.get(arr_name) {
//         Some(var_value) => {
//             match var_value.as_ref() {
//                 Value::Array(arr) => {
//                     // arr_rc = arr.clone();
//                     // let arr_rc = &*(arr.as_ref().borrow());
//                     // let cloned = arr.clone();
//                     // let arr_rc: Ref<ArrayLiteral> = cloned.as_ref().borrow();
//                     arr_cloned = arr.clone();
//                     arr_rc = arr_cloned.as_ref().borrow();

//                     let index =
//                         index_resolve(&params[0], scope)?;
//                     if index >= arr_rc.len() {
//                         println!("Index out of range.");
//                         return Err(())
//                     }
//                     let is_nested_reading = params.len() > 1;
//                     match &arr_rc[index] {
//                         Value::Number(num) =>{
//                             if is_nested_reading {
//                                 println!("Number has no sub-element.");
//                                 return Err(())
//                             }
//                             break Value::Number(*num)
//                         },
//                         Value::Array(arr) => {
//                             if is_nested_reading {
//                                 params = params[1]
//                                     .params
//                                     .as_ref()
//                                     .unwrap();
//                             } else {
//                                 break Value::Array(arr.clone())
//                             }
//                         },
//                         _ => {
//                             println!("Unexpected element in array '{}'.", arr_name);
//                             return Err(())
//                         }
//                     };
//                 },
//                 _ => {
//                     println!("'{}' is not an valid array.", arr_name);
//                     return Err(())
//                 },
//             }
//         },
//         None => {
//             println!("Variable '{}' is undefined.", arr_name);
//             return Err(())
//         },
//     }};

//     Ok(Rc::new(result))
// }