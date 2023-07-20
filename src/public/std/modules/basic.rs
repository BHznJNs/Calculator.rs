use std::io::{self, Write};
use std::process;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::public::error::{internal_error, type_error, InternalComponent};
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
    FRACTION,
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
    let exit = BuildInFunction {
        params: vec![],
        identi: BuildInFnIdenti::Basic(BasicFn::EXIT),
    };
    let array = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Number, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::ARRAY),
    };
    let fraction = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Number, "upper"),
            BuildInFnParam(ValueType::Number, "lower"),
        ],
        identi: BuildInFnIdenti::Basic(BasicFn::FRACTION),
    };

    // --- --- --- --- --- ---

    let function_template = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Void, "input")],
        identi: BuildInFnIdenti::Basic(BasicFn::TYPE),
    };
    let type__ = function_template.clone();
    let mut clone = function_template.clone();
    let mut int = function_template.clone();
    let mut float = function_template.clone();
    let mut boolean = function_template.clone();
    let mut string = function_template.clone();
    let mut ascii = function_template.clone();
    let mut len = function_template.clone();
    clone.identi = BuildInFnIdenti::Basic(BasicFn::CLONE);
    int.identi = BuildInFnIdenti::Basic(BasicFn::INT);
    float.identi = BuildInFnIdenti::Basic(BasicFn::FLOAT);
    boolean.identi = BuildInFnIdenti::Basic(BasicFn::BOOLEAN);
    string.identi = BuildInFnIdenti::Basic(BasicFn::STRING);
    ascii.identi = BuildInFnIdenti::Basic(BasicFn::ASCII);
    len.identi = BuildInFnIdenti::Basic(BasicFn::LEN);

    return vec![
        (String::from("input"), Value::from(input)),
        (String::from("type"), Value::from(type__)),
        (String::from("clone"), Value::from(clone)),
        (String::from("int"), Value::from(int)),
        (String::from("float"), Value::from(float)),
        (String::from("fraction"), Value::from(fraction)),
        (String::from("bool"), Value::from(boolean)),
        (String::from("string"), Value::from(string)),
        (String::from("array"), Value::from(array)),
        (String::from("ascii"), Value::from(ascii)),
        (String::from("len"), Value::from(len)),
        (String::from("exit"), Value::from(exit)),
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
            BasicFn::FRACTION => {
                let upper_value = get_val("upper", scope)?;
                let lower_value = get_val("lower", scope)?;

                if let (Value::Number(Number::Int(upper)), Value::Number(Number::Int(lower))) =
                    (upper_value, lower_value)
                {
                    return Ok(Value::Number(Number::Fraction(upper, lower)));
                } else {
                    return Err(internal_error(
                        InternalComponent::Std,
                        "two Int typed value is expected",
                    )?);
                }
            }
            BasicFn::EXIT => process::exit(0),

            _ => {
                let input = get_val("input", scope)?;

                match self {
                    BasicFn::TYPE => Value::from(input.get_type() as i64),
                    BasicFn::CLONE => input.deep_clone(),

                    BasicFn::INT => match input {
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
                    },
                    BasicFn::FLOAT => match input {
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
                    },

                    BasicFn::BOOLEAN => Value::Boolean(input.get_bool()),
                    BasicFn::STRING => Value::from(input.to_raw_string()),
                    BasicFn::ARRAY => {
                        let Value::Number(num) = input else {
                            unreachable!()
                        };
                        let size = num.int_value() as usize;
                        let arr_literal: ArrayLiteral = vec![Value::from(0); size].into();
                        Value::from(arr_literal)
                    }
                    BasicFn::ASCII => {
                        let input_ref = input.get_str()?;
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
                        if let Value::Array(arr) = input {
                            let refer = arr.borrow();
                            Value::Number(Number::Int(refer.len() as i64))
                        } else if let Value::String(str) = input {
                            let refer = str.borrow();
                            Value::Number(Number::Int(refer.len() as i64))
                        } else {
                            Value::Void(VoidSign::Empty)
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };
        return Ok(result);
    }
}
