use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::{BuildInFnCall, ClassModule};

#[derive(PartialEq, Clone)]
pub enum ArrayModule {
    PUSH,
    POP,
    SHIFT,
    UNSHIFT,
    INSERT,
    REMOVE,
    JOIN,
}

static mut MODULE_CLASS: Option<Rc<Class>> = None;
impl ClassModule for ArrayModule {
    fn __static_class_init() {
        let push = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "element"),
            ],
            identi: BuildInFnIdenti::Array(Self::PUSH),
        };

        let pop = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Object, "self")],
            identi: BuildInFnIdenti::Array(Self::POP),
        };

        let shift = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Object, "self")],
            identi: BuildInFnIdenti::Array(Self::SHIFT),
        };
        let unshift = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Void, "element"),
            ],
            identi: BuildInFnIdenti::Array(Self::UNSHIFT),
        };
        let insert = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "index"),
                BuildInFnParam(ValueType::Void, "element"),
            ],
            identi: BuildInFnIdenti::Array(Self::INSERT),
        };
        let remove = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "index"),
            ],
            identi: BuildInFnIdenti::Array(Self::REMOVE),
        };
        let join = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "divider"),
            ],
            identi: BuildInFnIdenti::Array(Self::JOIN),
        };

        // --- --- --- --- --- ---

        unsafe {
            MODULE_CLASS = Some(
                Class::new(
                    vec![Property(ValueType::Array, String::from("v"))],
                    vec![
                        (String::from("push"), Function::from(push)),
                        (String::from("pop"), Function::from(pop)),
                        (String::from("shift"), Function::from(shift)),
                        (String::from("unshift"), Function::from(unshift)),
                        (String::from("insert"), Function::from(insert)),
                        (String::from("remove"), Function::from(remove)),
                        (String::from("join"), Function::from(join)),
                    ],
                )
                .into(),
            )
        }
    }

    fn module_class() -> Rc<Class> {
        if unsafe { MODULE_CLASS.is_none() } {
            Self::__static_class_init();
        }
        let class = unsafe { MODULE_CLASS.as_ref().unwrap().clone() };
        return class;
    }
}

impl BuildInFnCall for ArrayModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let arr_value = get_self_prop(&self_value, "v")?;
        let Value::Array(arr) = arr_value else {
            unreachable!()
        };
        let mut arr_ref = arr.borrow_mut();

        let result = match self {
            Self::PUSH => {
                let element_value = get_val("element", scope)?;
                arr_ref.push(element_value.clone());
                element_value
            }
            Self::POP => {
                let poped_el = arr_ref.pop();
                if let Some(val) = poped_el {
                    // return poped value
                    return Ok(val);
                }
                Value::Void(VoidSign::Empty)
            }
            Self::SHIFT => {
                let shifted = arr_ref.shift();
                if let Some(val) = shifted {
                    // return shifted value
                    return Ok(val);
                }
                Value::Void(VoidSign::Empty)
            }
            Self::UNSHIFT => {
                let element_value = get_val("element", scope)?;
                arr_ref.unshift(element_value.clone());
                element_value
            }
            Self::INSERT => {
                let index_value = get_val("index", scope)?;
                let element_value = get_val("element", scope)?;

                let index = index_value.get_i64()? as usize;
                arr_ref.insert(index, element_value.clone());
                element_value
            }
            Self::REMOVE => {
                let index_value = get_val("index", scope)?;

                let index = index_value.get_i64()? as usize;
                let removed_element = arr_ref.remove(index);
                match removed_element {
                    Some(val) => val,
                    None => Value::Void(VoidSign::Empty),
                }
            }
            Self::JOIN => {
                let divider_value = get_val("divider", scope)?;
                let divider_ref = divider_value.get_str()?;
                let result_str = arr_ref.join(&*divider_ref);
                Value::from(result_str)
            }
        };
        return Ok(result);
    }
}
