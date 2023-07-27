use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::public::env::ENV_OPTION;
use crate::public::error::{assignment_error, reference_error, ReferenceType};
use crate::public::value::{GetAddr, self};
use crate::public::value::array::Array;
use crate::public::value::oop::class::Class;
use crate::public::value::value::VoidSign;
use crate::utils::completer::Completer;

use super::super::display_indent;
use super::super::value::Value;
use super::data_storage::{ComposeStorage, DataStoragePattern, ListStorage};

#[derive(PartialEq, Clone)]
pub enum Object {
    BuildIn(BuildInObject),
    UserDefined(UserDefinedObject),
}

type Prototype = Rc<Class>;

impl Object {
    pub fn new(params: ListStorage<Value>, prototype: Option<Prototype>) -> Self {
        match prototype {
            Some(proto) => {
                let store = ComposeStorage::new(params);
                Object::UserDefined(UserDefinedObject {
                    prototype: proto,
                    storage: store,
                })
            }
            None => {
                let completer = {
                    if unsafe { ENV_OPTION.is_repl } {
                        let mut words = vec![];
                        let mut temp = Completer::new();
                        for (k, _) in &params {
                            words.push(k.as_str())
                        }
                        temp.extend(words);
                        Some(temp.into())
                    } else {
                        None
                    }
                };
                let storage = ComposeStorage::new(params);
                Object::BuildIn(BuildInObject { completer, storage })
            }
        }
    }

    fn get_store<'obj>(&'obj self) -> &'obj ComposeStorage<Value> {
        match self {
            Object::BuildIn(obj) => &obj.storage,
            Object::UserDefined(obj) => &obj.storage,
        }
    }
    pub fn get_proto(&self) -> Option<Prototype> {
        match self {
            Object::BuildIn(_) => None,
            Object::UserDefined(obj) => Some(obj.prototype.clone()),
        }
    }
    pub fn get_completer(&self) -> Option<Rc<Completer>> {
        match self {
            Object::BuildIn(obj) => obj.completer.clone(),
            Object::UserDefined(_) => {
                let Some(proto) = self.get_proto() else {
                    return None;
                };
                proto.completer.clone()
            }
        }
    }

    pub fn get(&self, prop_name: &str) -> Result<Value, ()> {
        let store = self.get_store();
        let target_value_result = store.getter(prop_name);

        match target_value_result {
            Ok(target_ref) => Ok(target_ref.unwrap()),
            Err(_) => {
                if let Some(prototype) = self.get_proto() {
                    let target_method = prototype.get_method(prop_name)?;
                    Ok(Value::Function(target_method.clone()))
                } else {
                    Ok(Value::Void(VoidSign::Empty))
                }
            }
        }
    }
    pub fn set(&mut self, prop_name: &str, value: Value) -> Result<(), ()> {
        let store = match self {
            Object::BuildIn(obj) => {
                let target_value = obj.storage.getter(prop_name);
                if let Ok(Value::Function(_)) = target_value {
                    return Err(assignment_error(
                        "invalid assignment to module object method",
                    )?);
                }
                &mut obj.storage
            }
            Object::UserDefined(obj) => &mut obj.storage,
        };

        let result = store.setter(prop_name, value);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(reference_error(ReferenceType::Property, prop_name)?),
        }
    }
}

impl GetAddr for Object {
    fn get_addr(&self) -> value::Addr {
        let ptr = self as *const Object;
        return ptr as value::Addr;
    }
}

#[derive(PartialEq, Clone)]
pub struct UserDefinedObject {
    prototype: Prototype,
    pub(self) storage: ComposeStorage<Value>,
}

#[derive(PartialEq, Clone)]
pub struct BuildInObject {
    completer: Option<Rc<Completer>>,
    pub(self) storage: ComposeStorage<Value>,
}

pub fn display(f: &mut fmt::Formatter<'_>, obj: &Rc<RefCell<Object>>, level: usize) -> fmt::Result {
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
            Value::Object(obj) => self::display(f, &obj, level + 1)?,
            _ => write!(f, "{}", value)?,
        }

        // next line
        write!(f, "\r\n")
    }

    let obj_ref = obj.as_ref().borrow();
    let store = obj_ref.get_store();
    let ComposeStorage {
        storage_pattern,
        data_list,
        data_map,
    } = store;

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
    if let Some(proto) = obj_ref.get_proto() {
        Class::display_methods(f, &proto, level)?;
    }
    write!(f, "{}}}", display_indent(level - 1))
}

pub fn deep_clone(obj: Rc<RefCell<Object>>) -> Value {
    fn item_resolve(v: &Value) -> Value {
        if let Value::Object(sub_obj) = v.unwrap() {
            return self::deep_clone(sub_obj);
        } else {
            return v.deep_clone();
        }
    }

    let obj_ref = &*(obj.as_ref().borrow());
    let store = obj_ref.get_store();
    let ComposeStorage {
        storage_pattern,
        data_list,
        data_map,
    } = store;
    let mut instantiation_params = Vec::<(String, Value)>::new();

    match storage_pattern {
        DataStoragePattern::List => {
            let list = data_list.as_ref().unwrap();
            for (k, v) in list {
                let resolved_item_value = item_resolve(v);
                instantiation_params.push((k.clone(), resolved_item_value));
            }
        }
        DataStoragePattern::Map => {
            let map = data_map.as_ref().unwrap();
            for (k, v) in map {
                let resolved_item_value = item_resolve(v);
                instantiation_params.push((k.clone(), resolved_item_value));
            }
        }
    }

    // the object has been passed the type check before,
    // thus with properties of the object,
    // the instantiation must pass the type check.
    let res_obj = match obj_ref {
        Object::BuildIn(_) => Object::new(instantiation_params, None),
        Object::UserDefined(_) => {
            Object::new(instantiation_params, Some(obj_ref.get_proto().unwrap()))
        }
    };
    return Value::from(res_obj);
}
