use std::collections::VecDeque;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{
    BuildInFnParam, BuildInFunction, Function,
};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::{Value, ValueType};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
pub enum StringFn {
    SPLIT,
    REPLACE,
    REPEAT,
    STARTWITH,
    ENDWITH,
}

pub fn module_class() -> Class {
    let split = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "divider"),
        ],
        identi: BuildInFnIdenti::String(StringFn::SPLIT),
    };
    let replace = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "from"),
            BuildInFnParam(ValueType::Void, "to"),
        ],
        identi: BuildInFnIdenti::String(StringFn::REPLACE),
    };
    let repeat = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "num"),
        ],
        identi: BuildInFnIdenti::String(StringFn::REPEAT),
    };
    let start_with = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "pat"),
        ],
        identi: BuildInFnIdenti::String(StringFn::STARTWITH),
    };
    let end_with = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "pat"),
        ],
        identi: BuildInFnIdenti::String(StringFn::ENDWITH),
    };

    return Class::new(
        vec![Property(ValueType::String, String::from("_"))],
        vec![
            (String::from("split"), Function::from(split.clone())),
            (String::from("replace"), Function::from(replace.clone())),
            (String::from("repeat"), Function::from(repeat.clone())),
            (String::from("start_with"), Function::from(start_with.clone())),
            (String::from("end_with"), Function::from(end_with.clone())),

            (String::from("分割"), Function::from(split)),
            (String::from("替换"), Function::from(replace)),
            (String::from("重复"), Function::from(repeat)),
            (String::from("以为始"), Function::from(start_with)),
            (String::from("以为终"), Function::from(end_with)),
        ],
    );
}

impl BuildInFnCall for StringFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let str_value = get_self_prop(&self_value, "_")?;
        let str_ref = str_value.get_str()?;

        let result = match self {
            StringFn::SPLIT => {
                let divider_value = get_val("divider", scope)?;
                let divider_ref = divider_value.get_str()?;

                // splited chars
                let res_split = if divider_ref.is_empty() {
                    str_ref.split(' ')
                } else {
                    let first_ch = divider_ref.chars().next().unwrap();
                    str_ref.split(first_ch)
                };
                // convert splited to VecDeque<String>
                let mut res_vec = VecDeque::new();
                for c in res_split {
                    let c_value = Value::from(c.to_string());
                    res_vec.push_back(c_value);
                }
                Value::from(res_vec)
            }

            StringFn::REPLACE => {
                let from_value = get_val("from", scope)?;
                let to_value = get_val("to", scope)?;
                let (from_ref, to_ref) = (from_value.get_str()?, to_value.get_str()?);
                let replaced_str = str_ref.replace(&*from_ref, &to_ref);
                Value::from(replaced_str)
            }
            StringFn::REPEAT => {
                let num_value = get_val("num", scope)?;
                let repeat_count = num_value.get_i64()?;
                let repeated_str = str_ref.repeat(repeat_count as usize);
                Value::from(repeated_str)
            }

            StringFn::STARTWITH | StringFn::ENDWITH => {
                let pat_value = get_val("pat", scope)?;
                let pat_ref = pat_value.get_str()?;
                let result =
                match self {
                    StringFn::STARTWITH => str_ref.starts_with(&*pat_ref),
                    StringFn::ENDWITH => str_ref.ends_with(&*pat_ref),
                    _ => unreachable!()
                };
                Value::from(result)
            }
        };
        return Ok(result);
    }
}
