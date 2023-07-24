use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::public::error::{reference_error, ReferenceType};
use crate::public::value::array::{Array, ArrayLiteral};
use crate::public::value::oop::class::Class;

use super::super::display_indent;
use super::super::value::Value;
use super::data_storage::{DataStoragePattern, ComposeStorage};

#[derive(PartialEq, Clone)]
pub struct Object {
    pub prototype: Rc<Class>,
    pub(super) storage: ComposeStorage<Value>,
}

impl Object {
    pub fn get(&self, prop_name: &str) -> Result<Value, ()> {
        let target_value_result = self.storage.getter(prop_name);
        match target_value_result {
            Ok(target_ref) => {
                Ok(target_ref.unwrap())
            }
            Err(_) => {
                let target_method = self.prototype.get_method(prop_name)?;
                Ok(Value::Function(target_method.clone()))
            }
        }
    }

    pub fn set(&mut self, prop_name: &str, value: Value) -> Result<(), ()> {
        let result = self.storage.setter(prop_name, value);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(reference_error(ReferenceType::Property, prop_name)?)
        }
    }

    pub fn display(
        f: &mut fmt::Formatter<'_>,
        obj: &Rc<RefCell<Object>>,
        level: usize,
    ) -> fmt::Result {
        fn display_item(
            f: &mut fmt::Formatter<'_>,
            key: &str,
            value: &Value,
            level: usize,
        ) -> fmt::Result {
            // print indent and key
            write!(f, "{}{}: ", display_indent(level), key)?;

            // print value
            match value {
                Value::String(_) => write!(f, "{}", value.str_format())?,
                Value::Array(arr) => Array::display(f, &arr, level + 1)?,
                Value::Object(obj) => Object::display(f, &obj, level + 1)?,
                _ => write!(f, "{}", value)?,
            }

            // next line
            write!(f, "\r\n")
        }

        let obj_ref = obj.as_ref().borrow();
        let ComposeStorage {storage_pattern, data_list, data_map} = &obj_ref.storage;

        write!(f, "{{\r\n")?;
        match storage_pattern {
            DataStoragePattern::List => {
                let list = data_list.as_ref().unwrap();
                for (k, v) in list {
                    display_item(f, k, v, level)?;
                }
            }
            DataStoragePattern::Map => {
                let map = data_map.as_ref().unwrap();

                for (k, v) in map {
                    display_item(f, k, v, level)?;
                }
            }
        }
        Class::display_methods(f, &obj_ref.prototype, level)?;
        write!(f, "{}}}", display_indent(level - 1))
    }

    pub fn deep_clone(obj: Rc<RefCell<Object>>) -> Value {
        fn prop_value_resolve(v: &Value, param_vec: &mut ArrayLiteral) {
            if let Value::Object(sub_obj) = v.unwrap() {
                param_vec.push_back(Object::deep_clone(sub_obj));
            } else {
                param_vec.push_back(v.deep_clone());
            }
        }

        let obj_ref = &*(obj.as_ref().borrow());
        let ComposeStorage {storage_pattern, data_list, data_map} = &obj_ref.storage;
        let mut instantiation_params = ArrayLiteral::new();

        match storage_pattern {
            DataStoragePattern::List => {
                let list = data_list.as_ref().unwrap();
                for (_, v) in list {
                    prop_value_resolve(v, &mut instantiation_params);
                }
            }
            DataStoragePattern::Map => {
                let map = data_map.as_ref().unwrap();
                for (_, v) in map {
                    prop_value_resolve(v, &mut instantiation_params);
                }
            }
        }

        // the object has been passed the type check before,
        // thus with properties of the object,
        // the instantiation must pass the type check.
        let res_object =
            Class::instantiate(obj_ref.prototype.clone(), instantiation_params).unwrap();
        return Value::from(res_object);
    }
}
