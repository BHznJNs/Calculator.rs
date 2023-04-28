use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::number::Number;
use crate::public::value::value::{ValueTypes, Value};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match func_body {
        BuildInFuncs::Push => {
            let arr_value = get_val("arr", scope)?;
            let element_value = get_val("element", scope)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.push_back(element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Pop => {
            let arr_value = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.pop_back();
            }
            Value::Number(Number::Empty)
        },
        BuildInFuncs::Shift => {
            let arr_value = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.pop_front();
            }
            Value::Number(Number::Empty)
        },
        BuildInFuncs::Unshift => {
            let arr_value = get_val("arr", scope)?;
            let element_value = get_val("element", scope)?;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.push_front(element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Insert => {
            let arr_value = get_val("arr", scope)?;
            let index_value = get_val("index", scope)?;
            let element_value = get_val("element", scope)?;

            let index = index_value.get_i64()? as usize;

            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                refer.insert(index, element_value.clone());
            }
            element_value
        },
        BuildInFuncs::Remove => {
            let arr_value = get_val("arr", scope)?;
            let index_value = get_val("index", scope)?;

            let index = index_value.get_i64()? as usize;
            let mut removed_element: Option<Value> = None;
    
            if let Value::Array(arr) = arr_value {
                let mut refer = arr.borrow_mut();
                removed_element = refer.remove(index);
            }
            match removed_element {
                Some(val) => val,
                None => Value::Number(Number::Empty)
            }
        },
        BuildInFuncs::Len => {
            let arr_value = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_value {
                let refer = arr.borrow();
                Value::Number(Number::Int(refer.len() as i64))
            } else
            if let Value::String(str) = arr_value {
                let refer = str.borrow();
                Value::Number(Number::Int(refer.len() as i64))
            } else {
                Value::Number(Number::Empty)
            }
        },
        _ => {
            println!("Unexpected function in array implement.");
            return Err(())
        }
    };
    Ok(result)
}

pub fn function_list() -> Vec<(&'static str, Rc<BuildInFunction>)> {
    vec![
        ("push"   , Rc::new(PUSH)),
        ("pop"    , Rc::new(POP)),
        ("shift"  , Rc::new(SHIFT)),
        ("unshift", Rc::new(UNSHIFT)),
        ("insert" , Rc::new(INSERT)),
        ("remove" , Rc::new(REMOVE)),
        ("len"    , Rc::new(LEN)),
    ]
}

pub const PUSH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        }), None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Push,
};

pub const POP: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Pop,
};

pub const SHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Shift,
};
pub const UNSHIFT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        }), None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Unshift,
};
pub const INSERT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "index"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "element"
        })
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Insert,
};
pub const REMOVE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "index"
        }), None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Remove,
};
pub const LEN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Len,
};