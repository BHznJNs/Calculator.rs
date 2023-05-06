use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction, Function, Overload};
use crate::public::value::number::Number;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueTypes, Value};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    fn get_self_v(self_value: Value) -> Result<Value, ()> {
        let Value::Object(obj) = self_value else {
            println!("Invalid array getter invocation.");
            return Err(())
        };

        let obj_ref = obj.as_ref().borrow();
        Object::get(&obj_ref, &String::from("v"))
    }

    let result = match func_body {
        BuildInFuncs::Push => {
            let self_value = get_val("self", scope)?;
            let arr_value = get_self_v(self_value)?;
            let element_value = get_val("element", scope)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.push_back(element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Pop => {
            let self_value = get_val("self", scope)?;
            let arr_value = get_self_v(self_value)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                // return poped value
                let poped_el = refer.pop_back();
                if let Some(val) = poped_el {
                    return Ok(val)
                }
            }
            Value::Number(Number::Empty)
        },
        BuildInFuncs::Shift => {
            let self_value = get_val("self", scope)?;
            let arr_value = get_self_v(self_value)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                // return shifted value
                let shifted = refer.pop_front();
                if let Some(val) = shifted {
                    return Ok(val)
                }
            }
            Value::Number(Number::Empty)
        },
        BuildInFuncs::Unshift => {
            let self_value = get_val("self", scope)?;
            let element_value = get_val("element", scope)?;

            let arr_value = get_self_v(self_value)?;
            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.push_front(element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Insert => {
            let self_value = get_val("self", scope)?;
            let index_value = get_val("index", scope)?;
            let element_value = get_val("element", scope)?;

            let index = index_value.get_i64()? as usize;
            let arr_value = get_self_v(self_value)?;
            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.insert(index, element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Remove => {
            let self_value = get_val("self", scope)?;
            let index_value = get_val("index", scope)?;

            let index = index_value.get_i64()? as usize;
            let mut removed_element: Option<Value> = None;
            let arr_value = get_self_v(self_value)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                removed_element = refer.remove(index);
            }
            match removed_element {
                Some(val) => val,
                None => Value::Number(Number::Empty)
            }
        },
        _ => {
            println!("Unexpected function in array implement.");
            return Err(())
        }
    };
    Ok(result)
}

pub fn module_class() -> Class {
    Class {
        properties: vec![String::from("v")],
        method_storage: DataStoragePattern::Map,
        method_list: None,
        method_map: Some(HashMap::from([
            (String::from("push")   , Function::create(PUSH)),
            (String::from("pop")    , Function::create(POP)),
            (String::from("shift")  , Function::create(SHIFT)),
            (String::from("unshift"), Function::create(UNSHIFT)),
            (String::from("insert") , Function::create(INSERT)),
            (String::from("remove") , Function::create(REMOVE)),
        ]))
    }
}

pub const PUSH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        }), None, None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Push,
};

pub const POP: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), None, None, None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Pop,
};

pub const SHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), None, None, None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Shift,
};
pub const UNSHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        }), None, None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Unshift,
};
pub const INSERT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "index"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        }), None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Insert,
};
pub const REMOVE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "index"
        }), None, None,
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Remove,
};