use std::io::{self, Write};
use std::process;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::public::error::type_error;
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::str_to_num::str_to_num;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
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

    EXIT,
}

pub fn function_list() -> Vec<(String, Value)> {
    let input = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::String, "prompt")],
        identi: BuildInFnIdenti::Basic(BasicFn::INPUT),
    };
    // get value type
    let type__ = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::TYPE),
    };
    // deep clone value
    let clone = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::CLONE),
    };

    // Type converters
    let int = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::INT),
    };
    let float = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::FLOAT),
    };
    let boolean = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::BOOLEAN),
    };
    let string = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::STRING),
    };
    let array = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Number, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::ARRAY),
    };
    let ascii = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::String, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::ASCII),
    };
    let len = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::LEN),
    };
    let exit = BuildInFunction {
        params: vec![],
        identi: BuildInFnIdenti::Basic(BasicFn::EXIT),
    };

    return vec![
        (String::from("input"), Value::from(input.clone())),
        (String::from("type"), Value::from(type__.clone())),
        (String::from("clone"), Value::from(clone.clone())),
        (String::from("int"), Value::from(int.clone())),
        (String::from("float"), Value::from(float.clone())),
        (String::from("bool"), Value::from(boolean.clone())),
        (String::from("string"), Value::from(string.clone())),
        (String::from("array"), Value::from(array.clone())),
        (String::from("ascii"), Value::from(ascii.clone())),
        (String::from("len"), Value::from(len.clone())),
        (String::from("exit"), Value::from(exit.clone())),

        (String::from("输入"), Value::from(input)),
        (String::from("类型"), Value::from(type__)),
        (String::from("复杂"), Value::from(clone)),
        (String::from("整形"), Value::from(int)),
        (String::from("浮点形"), Value::from(float)),
        (String::from("布尔值"), Value::from(boolean)),
        (String::from("字符串"), Value::from(string)),
        (String::from("数组"), Value::from(array)),
        (String::from("ASCII"), Value::from(ascii)),
        (String::from("长度"), Value::from(len)),
        (String::from("退出"), Value::from(exit)),
    ];
}

impl BuildInFnCall for BasicFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            BasicFn::INPUT => {
                let prompt_value = get_val("prompt", scope)?;
                // show prompt
                let prompt_ref = prompt_value.get_str()?;
                print!("{}", prompt_ref);
                io::stdout().flush().unwrap();

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

                Value::from(input)
            }
            BasicFn::TYPE => {
                let input = get_val("input", scope)?;
                let type_value = input.get_type() as i64;
                Value::from(type_value)
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
                        Value::from(i)
                    }
                    Value::Boolean(bool_val) => Value::from(bool_val as i64),
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
                        Value::from(f)
                    }
                    Value::Boolean(bool_val) => Value::from(bool_val as i64 as f64),
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
                let result_bool = input.get_bool();
                Value::Boolean(result_bool)
            }
            BasicFn::STRING => {
                let input = get_val("input", scope)?;
                Value::from(input.to_raw_string())
            }
            BasicFn::ARRAY => {
                let input = get_val("input", scope)?;

                let Value::Number(num) = input else {
                    unreachable!()
                };
                let size = num.int_value() as usize;
                let arr_literal: ArrayLiteral = vec![Value::from(0); size].into();
                Value::from(arr_literal)
            }
            BasicFn::ASCII => {
                let input_value = get_val("input", scope)?;
                let input_ref = input_value.get_str()?;
                let Some(first_char) = input_ref.chars().next() else {
                    return Ok(Value::from(0));
                };

                if first_char.is_ascii() {
                    Value::from(first_char as i64)
                } else {
                    println!("Invalid ASCII character");
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
            BasicFn::EXIT => process::exit(0),
        };
        return Ok(result);
    }
}
