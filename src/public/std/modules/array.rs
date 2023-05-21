use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInFunction, BuildInParam, Function, Overload};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

pub fn module_class() -> Class {
    Class {
        properties: vec![Property {
            identi: String::from("v"),
            type__: ValueType::Array,
        }],
        method_storage: DataStoragePattern::Map,
        method_list: None,
        method_map: Some(HashMap::from([
            (String::from("push"), Function::create(PUSH)),
            (String::from("pop"), Function::create(POP)),
            (String::from("shift"), Function::create(SHIFT)),
            (String::from("unshift"), Function::create(UNSHIFT)),
            (String::from("insert"), Function::create(INSERT)),
            (String::from("remove"), Function::create(REMOVE)),
        ])),
    }
}

#[derive(PartialEq)]
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
        let result = match self {
            ArrayFn::PUSH => {
                let self_value = get_val("self", scope)?;
                let arr_value = get_self_prop(self_value, "v")?;
                let element_value = get_val("element", scope)?;

                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    refer.push_back(element_value.clone());
                }
                element_value
            }
            ArrayFn::POP => {
                let self_value = get_val("self", scope)?;
                let arr_value = get_self_prop(self_value, "v")?;

                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    // return poped value
                    let poped_el = refer.pop_back();
                    if let Some(val) = poped_el {
                        return Ok(val);
                    }
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::SHIFT => {
                let self_value = get_val("self", scope)?;
                let arr_value = get_self_prop(self_value, "v")?;

                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    // return shifted value
                    let shifted = refer.pop_front();
                    if let Some(val) = shifted {
                        return Ok(val);
                    }
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::UNSHIFT => {
                let self_value = get_val("self", scope)?;
                let element_value = get_val("element", scope)?;

                let arr_value = get_self_prop(self_value, "v")?;
                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    refer.push_front(element_value.clone());
                }
                element_value
            }
            ArrayFn::INSERT => {
                let self_value = get_val("self", scope)?;
                let index_value = get_val("index", scope)?;
                let element_value = get_val("element", scope)?;

                let index = index_value.get_i64()? as usize;
                let arr_value = get_self_prop(self_value, "v")?;
                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    refer.insert(index, element_value.clone());
                }
                element_value
            }
            ArrayFn::REMOVE => {
                let self_value = get_val("self", scope)?;
                let index_value = get_val("index", scope)?;

                let index = index_value.get_i64()? as usize;
                let mut removed_element: Option<Value> = None;
                let arr_value = get_self_prop(self_value, "v")?;

                if let Value::Array(arr) = arr_value {
                    let mut refer = arr.borrow_mut();
                    removed_element = refer.remove(index);
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

// --- --- --- --- --- ---

const PUSH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "element",
        }),
        None,
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::PUSH),
};

const POP: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::POP),
};

const SHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::SHIFT),
};
const UNSHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "element",
        }),
        None,
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::UNSHIFT),
};
const INSERT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "index",
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "element",
        }),
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::INSERT),
};
const REMOVE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self",
        }),
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "index",
        }),
        None,
        None,
    ],
    identi: BuildInFnIdenti::Array(ArrayFn::REMOVE),
};
