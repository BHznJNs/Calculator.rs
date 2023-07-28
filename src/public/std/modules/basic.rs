use std::cell::RefCell;
use std::io::{self, Write};
use std::process;
use std::rc::Rc;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::public::error::{internal_error, type_error, InternalComponent};
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::std::utils::str_to_num::str_to_num;
use crate::public::value::array::{ArrayLiteral, RawArray};
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ValueType};
use crate::public::value::GetAddr;

use super::super::utils::get_val::get_val;
use super::array::ArrayModule;
use super::string::StringModule;
use super::{BuildInFnCall, ClassModule, FunctionModule};

#[derive(PartialEq, Clone)]
pub enum BasicModule {
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

impl FunctionModule for BasicModule {
    fn function_list() -> Vec<(String, Value)> {
        let input = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::String, "prompt")],
            identi: BuildInFnIdenti::Basic(Self::INPUT),
        };
        let exit = BuildInFunction {
            params: vec![],
            identi: BuildInFnIdenti::Basic(Self::EXIT),
        };
        let array = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Number, "input")],
            identi: BuildInFnIdenti::Basic(Self::ARRAY),
        };
        let fraction = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Number, "upper"),
                BuildInFnParam(ValueType::Number, "lower"),
            ],
            identi: BuildInFnIdenti::Basic(Self::FRACTION),
        };

        // --- --- --- --- --- ---

        let function_template = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Void, "input")],
            identi: BuildInFnIdenti::Basic(Self::TYPE),
        };
        let type__ = function_template.clone();
        let mut clone = function_template.clone();
        let mut int = function_template.clone();
        let mut float = function_template.clone();
        let mut boolean = function_template.clone();
        let mut string = function_template.clone();
        let mut ascii = function_template.clone();
        let mut len = function_template.clone();
        clone.identi = BuildInFnIdenti::Basic(Self::CLONE);
        int.identi = BuildInFnIdenti::Basic(Self::INT);
        float.identi = BuildInFnIdenti::Basic(Self::FLOAT);
        boolean.identi = BuildInFnIdenti::Basic(Self::BOOLEAN);
        string.identi = BuildInFnIdenti::Basic(Self::STRING);
        ascii.identi = BuildInFnIdenti::Basic(Self::ASCII);
        len.identi = BuildInFnIdenti::Basic(Self::LEN);

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
}

impl BuildInFnCall for BasicModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            Self::INPUT => {
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
            Self::FRACTION => {
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
            Self::EXIT => process::exit(0),

            _ => {
                let input = get_val("input", scope)?;

                match self {
                    Self::TYPE => Value::from(input.get_type() as i64),
                    Self::CLONE => input.deep_clone(),

                    Self::INT => match input {
                        Value::Number(num) => Value::Number(num.int()),
                        Value::String(str) => {
                            let refer = str.as_ref().borrow();
                            let i = str_to_num::<i64>(refer)?;
                            Value::from(i)
                        }
                        Value::Boolean(bool_val) => Value::from(bool_val as i64),
                        _ => {
                            return Err(type_error(
                                Some("Build-in function `int`"),
                                vec![ValueType::Boolean, ValueType::Number, ValueType::String],
                                input.get_type(),
                            )?)
                        }
                    },
                    Self::FLOAT => match input {
                        Value::Number(num) => Value::Number(num.float()),
                        Value::String(str) => {
                            let refer = str.as_ref().borrow();
                            let f = str_to_num::<f64>(refer)?;
                            Value::from(f)
                        }
                        Value::Boolean(bool_val) => Value::from(bool_val as i64 as f64),
                        _ => {
                            return Err(type_error(
                                Some("Build-in function `float`"),
                                vec![ValueType::Boolean, ValueType::Number, ValueType::String],
                                input.get_type(),
                            )?)
                        }
                    },

                    Self::BOOLEAN => Value::Boolean(input.get_bool()),
                    Self::STRING => Value::from(input.to_raw_string()),
                    Self::ARRAY => {
                        let Value::Number(num) = input else {
                            unreachable!()
                        };
                        let size = num.int_value() as usize;
                        let arr_literal: ArrayLiteral = vec![Value::from(0); size].into();
                        Value::from(arr_literal)
                    }
                    Self::ASCII => {
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
                    Self::LEN => {
                        #[inline]
                        fn array_length(arr: &Rc<RefCell<RawArray>>) -> Value {
                            let refer = arr.borrow();
                            let length = refer.len() as i64;
                            return Value::from(length);
                        }
                        #[inline]
                        fn string_length(str: &Rc<RefCell<String>>) -> Value {
                            let refer = str.borrow();
                            let length = refer.chars().count() as i64;
                            return Value::from(length);
                        }
                        match &input {
                            Value::Array(arr) => return Ok(array_length(arr)),
                            Value::String(str) => return Ok(string_length(str)),
                            Value::Object(obj) => {
                                if let Some(proto) = obj.borrow().get_proto() {
                                    let string_cls = StringModule::module_class();
                                    let array_cls = ArrayModule::module_class();
                                    let proto_addr = proto.get_addr();

                                    if proto_addr == array_cls.get_addr() {
                                        let arr_val = get_self_prop(&input, "v")?;
                                        let Value::Array(arr_ref) = arr_val else {
                                            unreachable!()
                                        };
                                        return Ok(array_length(&arr_ref));
                                    }
                                    if proto_addr == string_cls.get_addr() {
                                        let str_val = get_self_prop(&input, "v")?;
                                        let Value::String(str_ref) = str_val else {
                                            unreachable!()
                                        };
                                        return Ok(string_length(&str_ref));
                                    }
                                }
                            }
                            _ => {}
                        };
                        return Err(type_error(
                            Some("Build-in function `len`"),
                            vec![ValueType::Array, ValueType::String],
                            input.get_type(),
                        )?);
                    }
                    _ => unreachable!(),
                }
            }
        };
        return Ok(result);
    }
}
