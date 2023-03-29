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
                        refer.push(Value::Number(*num)),
                    Value::Array(child_arr) =>
                        refer.push(Value::Array(child_arr.clone())),
                    _ => {}
                }
            }
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Pop => {
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Shift => {
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Unshift => {
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Insert => {
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Remove => {
            Value::Number(Number::Empty(None))
        },
        BuildInFuncs::Update => {
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
            type__: ValueTypes::Number,
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
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Unshift,
};
pub const INSERT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Insert,
};
pub const REMOVE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
    ],
    lib: StdModules::Array, 
    body: BuildInFuncs::Remove,
};
pub const UPDATE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Array,
            identi: "arr"
        }), None, None
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