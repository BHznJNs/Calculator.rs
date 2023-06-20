use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function, Overload};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

// static SELF_STR: &str= "self";

pub fn module_class() -> Class {
    let push = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::PUSH),
    };

    let pop = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::Array(ArrayFn::POP),
    };

    let shift: BuildInFunction = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::Array(ArrayFn::SHIFT),
    };
    let unshift: BuildInFunction = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::UNSHIFT),
    };
    let insert = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "index"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::INSERT),
    };
    let remove: BuildInFunction = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "index"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::REMOVE),
    };

    // --- --- --- --- --- ---

    Class {
        properties: vec![Property {
            identi: String::from("v"),
            type__: ValueType::Array,
        }],
        method_storage: DataStoragePattern::Map,
        method_list: None,
        method_map: Some(HashMap::from([
            (String::from("push"), Function::create(push)),
            (String::from("pop"), Function::create(pop)),
            (String::from("shift"), Function::create(shift)),
            (String::from("unshift"), Function::create(unshift)),
            (String::from("insert"), Function::create(insert)),
            (String::from("remove"), Function::create(remove)),
        ])),
    }
}

#[derive(PartialEq, Clone)]
pub enum ArrayFn {
    PUSH,
    POP,
    SHIFT,
    UNSHIFT,
    INSERT,
    REMOVE,
}

impl BuildInFnCall for ArrayFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let arr_value = get_self_prop(&self_value, "v")?;

        let result = match self {
            ArrayFn::PUSH => {
                let element_value = get_val("element", scope)?;

                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    arr_ref.push_back(element_value.clone());
                }
                element_value
            }
            ArrayFn::POP => {
                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    // return poped value
                    let poped_el = arr_ref.pop_back();
                    if let Some(val) = poped_el {
                        return Ok(val);
                    }
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::SHIFT => {
                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    // return shifted value
                    let shifted = arr_ref.pop_front();
                    if let Some(val) = shifted {
                        return Ok(val);
                    }
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::UNSHIFT => {
                let element_value = get_val("element", scope)?;

                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    arr_ref.push_front(element_value.clone());
                }
                element_value
            }
            ArrayFn::INSERT => {
                let index_value = get_val("index", scope)?;
                let element_value = get_val("element", scope)?;

                let index = index_value.get_i64()? as usize;
                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    arr_ref.insert(index, element_value.clone());
                }
                element_value
            }
            ArrayFn::REMOVE => {
                let index_value = get_val("index", scope)?;

                let index = index_value.get_i64()? as usize;
                let mut removed_element: Option<Value> = None;

                if let Value::Array(arr) = arr_value {
                    let mut arr_ref = arr.borrow_mut();
                    removed_element = arr_ref.remove(index);
                }
                match removed_element {
                    Some(val) => val,
                    None => Value::Void(VoidSign::Empty),
                }
            }
        };
        Ok(result)
    }
}
