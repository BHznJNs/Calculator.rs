use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::array::Array;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::{Value, ValueType, VoidSign};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
pub enum ArrayFn {
    PUSH,
    POP,
    SHIFT,
    UNSHIFT,
    INSERT,
    REMOVE,
    JOIN,
}

pub fn module_class() -> Class {
    let push = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::PUSH),
    };

    let pop = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::Array(ArrayFn::POP),
    };

    let shift = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::Array(ArrayFn::SHIFT),
    };
    let unshift = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::UNSHIFT),
    };
    let insert = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "index"),
            BuildInFnParam(ValueType::Void, "element"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::INSERT),
    };
    let remove = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "index"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::REMOVE),
    };
    let join = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "divider"),
        ],
        identi: BuildInFnIdenti::Array(ArrayFn::JOIN),
    };


    // --- --- --- --- --- ---

    return Class::new(
        vec![Property(ValueType::Array, String::from("_"))],
        vec![
            (String::from("push"), Function::from(push.clone())),
            (String::from("pop"), Function::from(pop.clone())),
            (String::from("shift"), Function::from(shift.clone())),
            (String::from("unshift"), Function::from(unshift.clone())),
            (String::from("insert"), Function::from(insert.clone())),
            (String::from("remove"), Function::from(remove.clone())),
            (String::from("join"), Function::from(join.clone())),

            (String::from("尾插"), Function::from(push)),
            (String::from("尾出"), Function::from(pop)),
            (String::from("头出"), Function::from(shift)),
            (String::from("头插"), Function::from(unshift)),
            (String::from("插入"), Function::from(insert)),
            (String::from("移除"), Function::from(remove)),
            (String::from("组合"), Function::from(join)),
        ],
    );
}

impl BuildInFnCall for ArrayFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let arr_value = get_self_prop(&self_value, "_")?;
        let Value::Array(arr) = arr_value else {
            unreachable!()
        };
        let mut arr_ref = arr.borrow_mut();

        let result = match self {
            ArrayFn::PUSH => {
                let element_value = get_val("element", scope)?;
                arr_ref.push_back(element_value.clone());
                element_value
            }
            ArrayFn::POP => {
                let poped_el = arr_ref.pop_back();
                if let Some(val) = poped_el {
                    // return poped value
                    return Ok(val);
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::SHIFT => {
                let shifted = arr_ref.pop_front();
                if let Some(val) = shifted {
                    // return shifted value
                    return Ok(val);
                }
                Value::Void(VoidSign::Empty)
            }
            ArrayFn::UNSHIFT => {
                let element_value = get_val("element", scope)?;
                arr_ref.push_front(element_value.clone());
                element_value
            }
            ArrayFn::INSERT => {
                let index_value = get_val("index", scope)?;
                let element_value = get_val("element", scope)?;

                let index = index_value.get_i64()? as usize;
                arr_ref.insert(index, element_value.clone());
                element_value
            }
            ArrayFn::REMOVE => {
                let index_value = get_val("index", scope)?;

                let index = index_value.get_i64()? as usize;
                let removed_element = arr_ref.remove(index);
                match removed_element {
                    Some(val) => val,
                    None => Value::Void(VoidSign::Empty),
                }
            }
            ArrayFn::JOIN => {
                let divider_value = get_val("divider", scope)?;
                let divider_ref = divider_value.get_str()?;
                let result_str = Array::join(&*arr_ref, &*divider_ref);
                Value::from(result_str)
            }
        };
        return Ok(result);
    }
}
