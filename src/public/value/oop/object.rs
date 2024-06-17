use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::public::env::ENV;
use crate::public::error::{assignment_error, reference_error, CalcResult, ReferenceType};
use crate::public::value::oop::class::Class;
use crate::public::value::{self, ComplexStructure, GetAddr};
use crate::utils::completer::Completer;

use super::super::core::Value;
use super::super::display_indent;
use super::data_storage::{ComposeStorage, DataStoragePattern, ListStorage};

#[derive(PartialEq, Clone)]
pub enum Object {
    BuildIn(BuildInObject),
    UserDefined(UserDefinedObject),
}

type Prototype = Rc<Class>;

impl Object {
    pub fn new(params: ListStorage<Value>, prototype: Option<Prototype>) -> Self {
        if let Some(proto) = prototype {
            // user-defined object has prototype
            let store = ComposeStorage::new(params);
            return Self::UserDefined(UserDefinedObject {
                prototype: proto,
                storage: store,
            });
        }

        let completer = if unsafe { ENV.options.use_repl || ENV.options.use_editor } {
            let mut words = vec![];
            let mut temp = Completer::new();
            for (k, _) in &params {
                words.push(k.as_str())
            }
            temp.extend(words);
            Some(temp.into())
        } else {
            None
        };
        let storage = ComposeStorage::new(params);
        return Self::BuildIn(BuildInObject { completer, storage });
    }

    fn get_store(&self) -> &ComposeStorage<Value> {
        match self {
            Self::BuildIn(obj) => &obj.storage,
            Self::UserDefined(obj) => &obj.storage,
        }
    }
    pub fn get_proto(&self) -> Option<Prototype> {
        match self {
            Self::BuildIn(_) => None,
            Self::UserDefined(obj) => Some(obj.prototype.clone()),
        }
    }
    pub fn get_completer(&self) -> Option<Rc<Completer>> {
        match self {
            Self::BuildIn(obj) => obj.completer.clone(),
            Self::UserDefined(_) => {
                let proto = self.get_proto()?;
                proto.completer.clone()
            }
        }
    }

    pub fn get(&self, prop_name: &str) -> CalcResult<Value> {
        let store = self.get_store();
        let target_value_result = store.getter(prop_name);

        match target_value_result {
            Some(target_ref) => Ok(target_ref.unwrap()),
            None => {
                if let Some(prototype) = self.get_proto() {
                    let target_method = prototype.get_method(prop_name)?;
                    Ok(Value::Function(target_method.clone()))
                } else {
                    Ok(Value::EMPTY)
                }
            }
        }
    }
    pub fn set(&mut self, prop_name: &str, value: Value) -> CalcResult<()> {
        let store = match self {
            Self::BuildIn(obj) => {
                let target_value = obj.storage.getter(prop_name);
                if let Some(Value::Function(_)) = target_value {
                    return Err(assignment_error(
                        "invalid assignment to module object method",
                    ));
                }
                &mut obj.storage
            }
            Self::UserDefined(obj) => &mut obj.storage,
        };

        let result = store.setter(prop_name, value);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(reference_error(ReferenceType::Property, prop_name)),
        }
    }
}

impl GetAddr for Object {
    fn get_addr(&self) -> value::Addr {
        let ptr = self as *const Self;
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

impl BuildInObject {
    pub fn null() -> Self {
        Self {
            completer: None,
            storage: ComposeStorage::new(vec![]),
        }
    }
    pub fn get_method_list(&self) -> Vec<&str> {
        match self.storage.storage_pattern {
            DataStoragePattern::List => self
                .storage
                .data_list
                .as_ref()
                .unwrap()
                .iter()
                .map(|item| item.0.as_str())
                .collect::<Vec<&str>>(),
            DataStoragePattern::Map => self
                .storage
                .data_map
                .as_ref()
                .unwrap()
                .keys()
                .map(|key| key.as_str())
                .collect(),
        }
    }
}

impl ComplexStructure for Object {
    fn display(f: &mut fmt::Formatter<'_>, obj: &Rc<RefCell<Self>>, level: usize) -> fmt::Result {
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
                    write!(f, "{}{}: ", display_indent(level), k)?;
                    Self::item_display(f, v, level + 1)?;
                    write!(f, "\r\n")?;
                }
            }
            DataStoragePattern::Map => {
                let map = data_map.as_ref().unwrap();
                for (k, v) in map {
                    write!(f, "{}{}: ", display_indent(level), k)?;
                    Self::item_display(f, v, level + 1)?;
                    write!(f, "\r\n")?;
                }
            }
        }
        if let Some(proto) = obj_ref.get_proto() {
            Class::display_methods(f, &proto, level)?;
        }
        write!(f, "{}}}", display_indent(level - 1))
    }

    fn deep_clone(obj: &Rc<RefCell<Self>>) -> Value {
        fn item_resolve(v: &Value) -> Value {
            if let Value::Object(sub_obj) = v {
                return Object::deep_clone(sub_obj);
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
}
