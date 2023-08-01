use std::rc::Rc;

use crate::public::{
    value::{
        oop::class::{Class, Property},
        value::{Value, ValueType, VoidSign}, function::{Function, BuildInFunction, BuildInFnParam}, array::ArrayLiteral,
    },
    run_time::{scope::Scope, build_in::BuildInFnIdenti}, std::utils::{get_val::get_val, get_self_prop::get_self_prop},
};

use super::{ClassModule, BuildInFnCall};

#[derive(PartialEq, Clone)]
pub enum MapModule {
    CLEAR,
    KEYS,
    VALUES,
    HASKEY,
}

static mut MODULE_CLASS: Option<Rc<Class>> = None;
impl ClassModule for MapModule {
    fn __static_class_init() {
        let clear = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Object, "self")],
            identi: BuildInFnIdenti::Map(Self::CLEAR),
        };
        // `clear` as function template
        let mut keys = clear.clone();
        let mut values = clear.clone();
        keys.identi = BuildInFnIdenti::Map(Self::KEYS);
        values.identi = BuildInFnIdenti::Map(Self::VALUES);

        let has_key = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "key_name"),
            ],
            identi: BuildInFnIdenti::Map(Self::HASKEY),
        };
        
        unsafe {
            MODULE_CLASS = Some(
                Class::new(
                    vec![Property(ValueType::Map, String::from("v"))],
                    vec![
                        (String::from("clear"), Function::from(clear)),
                        (String::from("keys"), Function::from(keys)),
                        (String::from("values"), Function::from(values)),
                        (String::from("has_key"), Function::from(has_key)),
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

impl BuildInFnCall for MapModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let map_value = get_self_prop(&self_value, "v")?;
        let Value::Map(map_temp) = map_value else {
            unreachable!()
        };
        let mut map_ref = map_temp.borrow_mut();

        let result = match self {
            MapModule::CLEAR => {
                map_ref.clear();
                Value::Void(VoidSign::Empty)
            }
            MapModule::KEYS => {
                let mut res_arr = ArrayLiteral::new();
                for key in map_ref.keys() {
                    res_arr.push_back(Value::from(key.to_owned()));
                }
                Value::from(res_arr)
            },
            MapModule::VALUES => {
                let mut res_arr = ArrayLiteral::new();
                for val in map_ref.values() {
                    res_arr.push_back(val.clone());
                }
                Value::from(res_arr)
            },
            MapModule::HASKEY => {
                let key_name_value = get_val("key_name", scope)?;
                let key_name = key_name_value.get_str()?;
                let is_has_key = map_ref.has_key(&*key_name);
                Value::from(is_has_key)
            },
        };
        return Ok(result);
    }
}
