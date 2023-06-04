use std::io::{self, Write};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::public::error::type_error;
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::str_to_num::str_to_num;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFunction, BuildInParam};
use crate::public::value::number::Number;
use crate::public::value::value::{Overload, Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

pub fn function_list() -> Vec<(String, Value)> {
    vec![
        (String::from("input"), Value::create(INPUT)),
        (String::from("type"), Value::create(TYPE)),
        (String::from("clone"), Value::create(CLONE)),
        (String::from("int"), Value::create(INT)),
        (String::from("float"), Value::create(FLOAT)),
        (String::from("bool"), Value::create(BOOLEAN)),
        (String::from("string"), Value::create(STRING)),
        (String::from("array"), Value::create(ARRAY)),
        (String::from("ascii"), Value::create(ASCII)),
        (String::from("len"), Value::create(LEN)),
    ]
}

#[derive(PartialEq)]
pub enum BasicFn {
    INPUT,
    TYPE,
    CLONE,

    INT,
    FLOAT,
    BOOLEAN,
    STRING,

    ARRAY,
    ASCII,
    LEN,
}

impl BuildInFnCall for BasicFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            BasicFn::INPUT => {
                let prompt = get_val("prompt", scope)?;
                // show prompt
                if let Value::String(str) = prompt {
                    print!("{}", str.as_ref().borrow());
                    io::stdout().flush().unwrap();
                }
                // get input
                let mut input = String::new();
                disable_raw_mode().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                enable_raw_mode().unwrap();

                // remove the "\r\n" at the end of input
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }

                Value::create(input)
            }
            BasicFn::TYPE => {
                let input = get_val("input", scope)?;
                let type_value = input.get_type() as i64;
                Value::create(type_value)
            }
            BasicFn::CLONE => {
                let input = get_val("input", scope)?;
                input.deep_clone()
            }

            BasicFn::INT => {
                let input = get_val("input", scope)?;

                match input {
                    Value::Number(num) => Value::Number(num.int()),
                    Value::String(str) => {
                        let refer = str.as_ref().borrow();
                        let i = str_to_num::<i64>(refer)?;
                        Value::create(i)
                    }
                    Value::Boolean(bool_val) => Value::create(bool_val as i64),
                    _ => {
                        return Err(type_error(
                            Some("Build-in function 'int'"),
                            vec![ValueType::Boolean, ValueType::Number, ValueType::String],
                            input.get_type(),
                        )?)
                    }
                }
            }
            BasicFn::FLOAT => {
                let input = get_val("input", scope)?;

                match input {
                    Value::Number(num) => Value::Number(num.float()),
                    Value::String(str) => {
                        let refer = str.as_ref().borrow();
                        let f = str_to_num::<f64>(refer)?;
                        Value::create(f)
                    }
                    Value::Boolean(bool_val) => Value::create(bool_val as i64 as f64),
                    _ => {
                        return Err(type_error(
                            Some("Build-in function 'float'"),
                            vec![ValueType::Boolean, ValueType::Number, ValueType::String],
                            input.get_type(),
                        )?)
                    }
                }
            }
            BasicFn::BOOLEAN => {
                let input = get_val("input", scope)?;

                let result_bool = match input {
                    Value::Void(_) => false,
                    Value::Boolean(bool_val) => bool_val,
                    Value::Number(num) => num.int_value() != 0,
                    Value::String(str) => !str.as_ref().borrow().is_empty(),
                    Value::Array(arr) => !arr.as_ref().borrow().is_empty(),

                    Value::Class(_)
                    | Value::Object(_)
                    | Value::Function(_)
                    | Value::LazyExpression(_) => true,
                };
                Value::Boolean(result_bool)
            }
            BasicFn::STRING => {
                let input = get_val("input", scope)?;

                match input {
                    Value::String(_) => input.deep_clone(),
                    Value::Number(num) => Value::create(num.to_string()),

                    Value::Boolean(bool_val) => Value::create(format!("{}", bool_val)),
                    _ => {
                        return Err(type_error(
                            Some("Build-in function 'string'"),
                            vec![ValueType::Boolean, ValueType::Number, ValueType::String],
                            input.get_type(),
                        )?)
                    }
                }
            }
            BasicFn::ARRAY => {
                let input = get_val("input", scope)?;

                if let Value::Number(num) = input {
                    let size = num.int_value() as usize;
                    let arr_literal: ArrayLiteral = vec![Value::create(0); size].into();
                    Value::create(arr_literal)
                } else {
                    return Err(type_error(
                        Some("Build-in function 'string'"),
                        vec![ValueType::Number],
                        input.get_type(),
                    )?);
                }
            }
            BasicFn::ASCII => {
                let input = get_val("input", scope)?;

                if let Value::String(str) = input {
                    let temp = str.as_ref().borrow();
                    let option_first_char = temp.chars().next();
                    if let Some(char) = option_first_char {
                        if char.is_ascii() {
                            Value::create(char as i64)
                        } else {
                            println!("Invalid ASCII character");
                            return Err(());
                        }
                    } else {
                        println!("Invalid params to convert.");
                        return Err(());
                    }
                } else {
                    println!("Invalid param type: expected String.");
                    return Err(());
                }
            }
            BasicFn::LEN => {
                let arr_value: Value = get_val("input", scope)?;

                if let Value::Array(arr) = arr_value {
                    let refer = arr.borrow();
                    Value::Number(Number::Int(refer.len() as i64))
                } else if let Value::String(str) = arr_value {
                    let refer = str.borrow();
                    Value::Number(Number::Int(refer.len() as i64))
                } else {
                    Value::Void(VoidSign::Empty)
                }
            }
        };
        Ok(result)
    }
}

// --- --- --- --- --- ---

const INPUT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::String,
            identi: "prompt",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::INPUT),
};
// get value type
const TYPE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::TYPE),
};
// deep clone value
const CLONE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::CLONE),
};

// Type converters
const INT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::INT),
};
const FLOAT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::FLOAT),
};
const BOOLEAN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::BOOLEAN),
};
const STRING: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::STRING),
};
const ARRAY: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::ARRAY),
};
const ASCII: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::String,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::ASCII),
};

const LEN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "input",
        }),
        None,
        None,
        None,
    ],
    identi: BuildInFnIdenti::Basic(BasicFn::LEN),
};
