use std::rc::Rc;

use crate::public::error::CalcResult;
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::{ModuleClass, EMPTY_MODULE_CLASS};
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::{Value, ValueType};

use super::super::utils::get_val::get_val;
use super::{BuildInFnCall, ClassModule};

#[derive(PartialEq, Clone)]
pub enum ArrayModule {
    Push,
    Pop,
    Shift,
    Unshift,
    Insert,
    Remove,
    Contains,
    Slice,
    Join,
}

pub static mut MODULE_CLASS: ModuleClass = EMPTY_MODULE_CLASS;
impl ClassModule for ArrayModule {
    fn __static_class__() -> Class {
        let push = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "element"),
            ],
            identi: BuildInFnIdenti::Array(Self::Push),
        };
        let pop = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Object, "self")],
            identi: BuildInFnIdenti::Array(Self::Pop),
        };
        let mut shift = pop.clone();
        let mut unshift = push.clone();
        shift.identi = BuildInFnIdenti::Array(Self::Shift);
        unshift.identi = BuildInFnIdenti::Array(Self::Unshift);

        let insert = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "index"),
                BuildInFnParam(ValueType::Void, "element"),
            ],
            identi: BuildInFnIdenti::Array(Self::Insert),
        };
        let remove = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "index"),
            ],
            identi: BuildInFnIdenti::Array(Self::Remove),
        };
        let contains = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "value"),
            ],
            identi: BuildInFnIdenti::Array(Self::Contains),
        };
        let slice = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "start"),
                BuildInFnParam(ValueType::Number, "end"),
            ],
            identi: BuildInFnIdenti::Array(Self::Slice),
        };
        let join = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "divider"),
            ],
            identi: BuildInFnIdenti::Array(Self::Join),
        };

        // --- --- --- --- --- ---

        return Class::new(
            vec![Property(ValueType::Array, String::from("v"))],
            vec![
                (String::from("push"), Function::from(push)),
                (String::from("pop"), Function::from(pop)),
                (String::from("shift"), Function::from(shift)),
                (String::from("unshift"), Function::from(unshift)),
                (String::from("insert"), Function::from(insert)),
                (String::from("remove"), Function::from(remove)),
                (String::from("contains"), Function::from(contains)),
                (String::from("slice"), Function::from(slice)),
                (String::from("join"), Function::from(join)),
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

impl BuildInFnCall for ArrayModule {
    fn call(&self, scope: &mut Scope) -> CalcResult<Value> {
        let self_value = get_val("self", scope)?;
        let arr_value = get_self_prop(&self_value, "v")?;
        let Value::Array(arr) = arr_value else {
            unreachable!()
        };
        let mut arr_ref = arr.borrow_mut();

        let result = match self {
            Self::Push => {
                let element_value = get_val("element", scope)?;
                arr_ref.push(element_value.clone());
                element_value
            }
            Self::Pop => {
                let poped_el = arr_ref.pop();
                if let Some(val) = poped_el {
                    // return poped value
                    return Ok(val);
                }
                Value::EMPTY
            }
            Self::Shift => {
                let shifted = arr_ref.shift();
                if let Some(val) = shifted {
                    // return shifted value
                    return Ok(val);
                }
                Value::EMPTY
            }
            Self::Unshift => {
                let element_value = get_val("element", scope)?;
                arr_ref.unshift(element_value.clone());
                element_value
            }
            Self::Insert => {
                let index_value = get_val("index", scope)?;
                let element_value = get_val("element", scope)?;

                let index = index_value.get_i64()? as usize;
                arr_ref.insert(index, element_value.clone());
                element_value
            }
            Self::Remove => {
                let index_value = get_val("index", scope)?;

                let index = index_value.get_i64()? as usize;
                let removed_element = arr_ref.remove(index);
                match removed_element {
                    Some(val) => val,
                    None => Value::EMPTY,
                }
            }
            Self::Contains => {
                let target_value = get_val("value", scope)?;
                let is_contains = arr_ref.contains(&target_value);
                Value::from(is_contains)
            }
            Self::Slice => {
                let start = get_val("start", scope)?.get_i64()?;
                let end = get_val("end", scope)?.get_i64()?;
                let slice = arr_ref.slice(start, end);
                Value::from(slice)
            }
            Self::Join => {
                let divider_value = get_val("divider", scope)?;
                let divider_ref = divider_value.get_str()?;
                let result_str = arr_ref.join(&divider_ref);
                Value::from(result_str)
            }
        };
        return Ok(result);
    }
}
