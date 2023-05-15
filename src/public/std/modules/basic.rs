use std::cell::Ref;
use std::io::{self, Write};
use std::str::FromStr;

use crate::public::run_time::build_in::BuildInFnEnum;
use crate::public::run_time::scope::Scope;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::number::Number;
use crate::public::value::value::{ValueType, Value, Overload};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFnEnum,
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
        BuildInFnEnum::Input => {
            let prompt =
                get_val("prompt", scope)?;
            // show prompt
            if let Value::String(str) = prompt {
                print!("{}", str.as_ref().borrow());
                io::stdout().flush().unwrap();
            }
            // get input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .unwrap();

            // remove the "\r\n" at the end of input
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            Value::create(input)
        },
        BuildInFnEnum::Type => {
            let input =
                get_val("input", scope)?;

            Value::create(input.get_type() as i64)
        },
        BuildInFnEnum::Clone => {
            let input =
                get_val("input", scope)?;
            input.deep_clone()?
        },

        BuildInFnEnum::Int => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input {
                let refer =
                    str.as_ref().borrow();
                let i = str_to_num::<i64>(refer)?;
                Value::create(i)
            } else
            if let Value::Number(num) = input {
                Value::Number(num.int())
            } else {
                println!("Invalid param type: expected String OR Number.");
                return Err(())
            }
        },
        BuildInFnEnum::Float => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input {
                let refer =
                    str.as_ref().borrow();
                let f = str_to_num::<f64>(refer)?;
                Value::create(f)
            } else
            if let Value::Number(num) = input {
                Value::Number(num.float())
            } else {
                println!("Invalid param type: expected String OR Number.");
                return Err(())
            }
        },
        BuildInFnEnum::String => {
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
        BuildInFnEnum::Array => {
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
        BuildInFnEnum::Ascii => {
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
        BuildInFnEnum::Len => {
            let arr_value: Value = get_val("input", scope)?;

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
            println!("Unexpected function in math implement.");
            return Err(())
        }
    };

    Ok(result)
}

pub fn function_list() -> Vec<(String, Value)> {
    vec![
        (String::from("input")  , Value::create(INPUT)),
        (String::from("type")   , Value::create(TYPE)),
        (String::from("clone")  , Value::create(CLONE)),
        (String::from("int")    , Value::create(INT)),
        (String::from("float")  , Value::create(FLOAT)),
        (String::from("string") , Value::create(STR)),
        (String::from("array")  , Value::create(ARRAY)),
        (String::from("ascii")  , Value::create(ASCII)),
        (String::from("len")    , Value::create(LEN)),
    ]
}

pub const INPUT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::String,
        identi: "prompt"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Input,
};
// get value type
pub const TYPE: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Void,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Type,
};
// deep clone value
pub const CLONE: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Void,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Clone,
};

// Type converters
pub const INT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Void,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Int,
};
pub const FLOAT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Void,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Float,
};
pub const STR: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Number,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::String,
};
pub const ARRAY: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Number,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic, 
    body: BuildInFnEnum::Array,
};
pub const ASCII: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::String,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic,
    body: BuildInFnEnum::Ascii,
};


pub const LEN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueType::Void,
        identi: "input"
    }), None, None, None,],
    lib: StdModules::Basic,
    body: BuildInFnEnum::Len,
};