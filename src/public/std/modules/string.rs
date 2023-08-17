use std::collections::VecDeque;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::{ModuleClass, EMPTY_MODULE_CLASS};
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::{Value, ValueType};

use super::super::utils::get_val::get_val;
use super::{BuildInFnCall, ClassModule};

#[derive(PartialEq, Clone)]
pub enum StringModule {
    SPLIT,
    REPLACE,
    REPEAT,
    STARTWITH,
    ENDWITH,
}

static mut MODULE_CLASS: ModuleClass = EMPTY_MODULE_CLASS;
impl ClassModule for StringModule {
    fn __static_class__() -> Class{
        let split = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "divider"),
            ],
            identi: BuildInFnIdenti::String(Self::SPLIT),
        };
        let replace = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "from"),
                BuildInFnParam(ValueType::Void, "to"),
            ],
            identi: BuildInFnIdenti::String(Self::REPLACE),
        };
        let repeat = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "num"),
            ],
            identi: BuildInFnIdenti::String(Self::REPEAT),
        };
        let start_with = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "pat"),
            ],
            identi: BuildInFnIdenti::String(Self::STARTWITH),
        };
        let end_with = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "pat"),
            ],
            identi: BuildInFnIdenti::String(Self::ENDWITH),
        };
        return Class::new(
            vec![Property(ValueType::String, String::from("v"))],
            vec![
                (String::from("split"), Function::from(split)),
                (String::from("replace"), Function::from(replace)),
                (String::from("repeat"), Function::from(repeat)),
                (String::from("start_with"), Function::from(start_with)),
                (String::from("end_with"), Function::from(end_with)),
            ],
        );
    }
    fn module_class() -> Rc<Class> {
        let class = unsafe {
            MODULE_CLASS.is_some_or_init(Self::__static_class__);
            MODULE_CLASS.unwrap()
        };
        return class;
    }
}

impl BuildInFnCall for StringModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let str_value = get_self_prop(&self_value, "v")?;
        let str_ref = str_value.get_str()?;

        let result = match self {
            Self::SPLIT => {
                let divider_value = get_val("divider", scope)?;
                let divider_ref = divider_value.get_str()?;

                // splited string
                let res_split = if divider_ref.is_empty() {
                    str_ref.split(" ")
                } else {
                    let div_str = divider_ref.as_str();
                    str_ref.split(div_str)
                };
                // convert splited to VecDeque<String>
                let mut res_vec = VecDeque::new();
                for item in res_split {
                    let c_value = Value::from(String::from(item));
                    res_vec.push_back(c_value);
                }
                Value::from(res_vec)
            }

            Self::REPLACE => {
                let from_value = get_val("from", scope)?;
                let to_value = get_val("to", scope)?;
                let (from_ref, to_ref) = (from_value.get_str()?, to_value.get_str()?);
                let replaced_str = str_ref.replace(&*from_ref, &to_ref);
                Value::from(replaced_str)
            }
            Self::REPEAT => {
                let num_value = get_val("num", scope)?;
                let repeat_count = num_value.get_i64()?;
                let repeated_str = str_ref.repeat(repeat_count as usize);
                Value::from(repeated_str)
            }

            Self::STARTWITH | Self::ENDWITH => {
                let pat_value = get_val("pat", scope)?;
                let pat_ref = pat_value.get_str()?;
                let result = match self {
                    Self::STARTWITH => str_ref.starts_with(&*pat_ref),
                    Self::ENDWITH => str_ref.ends_with(&*pat_ref),
                    _ => unreachable!(),
                };
                Value::from(result)
            }
        };
        return Ok(result);
    }
}
