use std::cell::Ref;
use std::rc::Rc;
use std::str::FromStr;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::value::{ValueTypes, Value, Overload, ArrayLiteral};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    fn str_to_num<T: FromStr>(
        str: Ref<String>
    ) -> Result<T, ()> {
        // i64 || f64
        match str.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => {
                println!("Invalid string coverting to number.");
                return Err(())
            },
        }
    }

    let result = match func_body {
        BuildInFuncs::Type => {
            let input =
                get_val("input", scope)?;

            Value::create(input.get_type() as i64)
        },
        BuildInFuncs::Clone => {
            let input =
                get_val("input", scope)?;
            input.deep_clone()?
        },

        BuildInFuncs::Int => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input {
                let refer = str.borrow();
                let i = str_to_num::<i64>(refer)?;
                Value::create(i)
            } else {
                println!("Invalid param type: expected String.");
                return Err(())
            }
        },
        BuildInFuncs::Float => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input {
                let refer = str.borrow();
                let f = str_to_num::<f64>(refer)?;
                Value::create(f)
            } else {
                println!("Invalid param type: expected String.");
                return Err(())
            }
        },
        BuildInFuncs::String => {
            let input =
                get_val("input", scope)?;

            if let Value::Number(num) = input {
                let str = num.to_string();
                Value::create(str)
            } else {
                println!("Invalid param type: expected Number.");
                return Err(())
            }
        },
        BuildInFuncs::Array => {
            let input =
                get_val("input", scope)?;

            if let Value::Number(num) = input {
                let size = num.int_value() as usize;
                let arr_literal: ArrayLiteral =
                    vec![Value::create(0); size].into();
                Value::create(arr_literal)
            } else {
                println!("Invalid param type: expected Number.");
                return Err(())
            }
        },
        BuildInFuncs::Ascii => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input {
                let temp = str.as_ref().borrow();
                let option_first_char = temp.chars().next();
                if let Some(char) = option_first_char {
                    if char.is_ascii() {
                        Value::create(char as i64)
                    } else {
                        println!("Invalid ASCII character");
                        return Err(())
                    }
                } else {
                    println!("Invalid params to convert.");
                    return Err(())
                }
            } else {
                println!("Invalid param type: expected String.");
                return Err(())
            }
        },
        _ => {
            println!("Unexpected function in math implement.");
            return Err(())
        }
    };

    Ok(result)
}

pub fn function_list() -> Vec<(&'static str, Rc<BuildInFunction>)> {
    vec![
        ("type"   , Rc::new(TYPE)),
        ("clone"  , Rc::new(CLONE)),
        ("int"    , Rc::new(INT)),
        ("float"  , Rc::new(FLOAT)),
        ("string" , Rc::new(STR)),
        ("array"  , Rc::new(ARRAY)),
        ("ascii"  , Rc::new(ASCII)),
    ]
}

// get value type
pub const TYPE: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Void,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Type,
};
// deep clone value
pub const CLONE: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Void,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Clone,
};

// Type converters
pub const INT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::String,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Int,
};
pub const FLOAT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::String,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Float,
};
pub const STR: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::String,
};
pub const ARRAY: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Array,
};
pub const ASCII: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::String,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic,
    body: BuildInFuncs::Ascii,
};