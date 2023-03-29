use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::number::Number;
use crate::public::value::value::{ValueTypes, Value};

use super::std::StdModules;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    fn get_val(
        val_name: &str,
        scope: &mut Scope
    ) -> Result<Rc<Value>, ()> {
        let val =
            scope.local
            .as_ref().unwrap()
            .variables
            .get(val_name);
        match val {
            Some(rc_val) =>
                Ok(rc_val.clone()),
            None => {
                println!("Input for function is missing.");
                Err(())
            },
        }
    }

    let result = match func_body {
        BuildInFuncs::Push => {
            let arr_rc = get_val("arr", scope)?;
            let element_rc = get_val("element", scope)?;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                match element_rc.as_ref() {
                    Value::Number(num) =>
                        refer.push_back(Value::Number(*num)),
                    Value::Array(child_arr) =>
                        refer.push_back(Value::Array(child_arr.clone())),
                    _ => {}
                }
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Pop => {
            let arr_rc = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                refer.pop_back();
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Shift => {
            let arr_rc = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                refer.pop_front();
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Unshift => {
            let arr_rc = get_val("arr", scope)?;
            let element_rc = get_val("element", scope)?;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                match element_rc.as_ref() {
                    Value::Number(num) =>
                        refer.push_front(Value::Number(*num)),
                    Value::Array(child_arr) =>
                        refer.push_front(Value::Array(child_arr.clone())),
                    _ => {}
                }
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Insert => {
            let arr_rc = get_val("arr", scope)?;
            let index_rc = get_val("index", scope)?;
            let element_rc = get_val("element", scope)?;

            let index = index_rc.get_i64()? as usize;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                match element_rc.as_ref() {
                    Value::Number(num) =>
                        refer.insert(index, Value::Number(*num)),
                    Value::Array(child_arr) =>
                        refer.insert(index, Value::Array(child_arr.clone())),
                    _ => {}
                }
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Remove => {
            let arr_rc = get_val("arr", scope)?;
            let index_rc = get_val("index", scope)?;

            let index = index_rc.get_i64()? as usize;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();
                refer.remove(index);
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Update => {
            let arr_rc = get_val("arr", scope)?;
            let index_rc = get_val("index", scope)?;
            let element_rc = get_val("element", scope)?;

            let index = index_rc.get_i64()? as usize;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let mut refer = arr.borrow_mut();

                if index >= refer.len() {
                    println!("Index out of range, current length: {}.", refer.len());
                    return Err(())
                }

                refer[index] = match element_rc.as_ref() {
                    Value::Number(num) =>
                        Value::Number(*num),
                    Value::Array(child_arr) =>
                        Value::Array(child_arr.clone()),
                    _ => {
                        println!("Invalid element type.");
                        return Err(())
                    }
                }
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Len => {
            let arr_rc = get_val("arr", scope)?;

            if let Value::Array(arr) = arr_rc.as_ref() {
                let refer = arr.borrow();
                Value::Number(Number::Int(refer.len() as i64))
            } else {
                Value::Number(Number::Empty(None))
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
        ("update" , Rc::new(UPDATE)),
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
            type__: ValueTypes::Number,
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
            type__: ValueTypes::Number,
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
            type__: ValueTypes::Number,
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
pub const UPDATE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "index"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "element"
        })
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Update,
};
pub const LEN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Len,
};