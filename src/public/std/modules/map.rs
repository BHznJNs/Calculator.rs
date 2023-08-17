use std::rc::Rc;

use crate::public::{
    run_time::{build_in::BuildInFnIdenti, scope::Scope},
    std::{utils::{get_self_prop::get_self_prop, get_val::get_val}, ModuleClass, EMPTY_MODULE_CLASS},
    value::{
        array::ArrayLiteral,
        function::{BuildInFnParam, BuildInFunction, Function},
        oop::class::{Class, Property},
        value::{Value, ValueType},
    },
};

use super::{BuildInFnCall, ClassModule};

#[derive(PartialEq, Clone)]
pub enum MapModule {
    CLEAR,
    KEYS,
    VALUES,
    HASKEY,
}

static mut MODULE_CLASS: ModuleClass = EMPTY_MODULE_CLASS;
impl ClassModule for MapModule {
    fn __static_class__() -> Class {
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

        return Class::new(
            vec![Property(ValueType::Map, String::from("v"))],
            vec![
                (String::from("clear"), Function::from(clear)),
                (String::from("keys"), Function::from(keys)),
                (String::from("values"), Function::from(values)),
                (String::from("has_key"), Function::from(has_key)),
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
                Value::EMPTY
            }
            MapModule::KEYS => {
                let mut res_arr = ArrayLiteral::new();
                for key in map_ref.keys() {
                    res_arr.push_back(Value::from(key.to_owned()));
                }
                Value::from(res_arr)
            }
            MapModule::VALUES => {
                let mut res_arr = ArrayLiteral::new();
                for val in map_ref.values() {
                    res_arr.push_back(val.clone());
                }
                Value::from(res_arr)
            }
            MapModule::HASKEY => {
                let key_name_value = get_val("key_name", scope)?;
                let key_name = key_name_value.get_str()?;
                let is_has_key = map_ref.has_key(&*key_name);
                Value::from(is_has_key)
            }
        };
        return Ok(result);
    }
}
