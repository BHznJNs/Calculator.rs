use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use super::super::value::Value;
use super::class::Class;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq, Clone)]
pub struct Object {
    pub prototype: Option<Rc<Class>>,

    pub storage_pattern: DataStoragePattern,
    pub data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>,
    pub data_map : Option<HashMap<String, Rc<RefCell<Value>>>>,
}

impl Object {
    pub fn get(&self, prop_name: &String) -> Result<Value, ()> {
        let target_value_result =
        getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );
        match target_value_result {
            Ok(target_rc) => {
                let target_ref =
                    target_rc.as_ref().borrow();
                Ok(target_ref.unwrap())
            },
            Err(_) => {
                match &self.prototype {
                    Some(proto) => {
                        let target_method =
                            proto.get_method(prop_name)?;
                        Ok(Value::Function(target_method.clone()))
                    },
                    None => {
                        println!("Property '{}' in object does not exist.", prop_name);
                        Err(())
                    }
                }
            },
        }
    }

    pub fn set(
        &self,
        prop_name: &String,
        value: Value
    ) -> Result<(), ()> {
        let result_target_rc =
        getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );

        match result_target_rc {
            Ok(target_rc) => {
                let mut target_ref =
                    target_rc.as_ref().borrow_mut();
                *target_ref = value;
                Ok(())
            },
            Err(err_msg) => {
                println!("{}", err_msg);
                Err(())
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{{");
        match self.storage_pattern {
            DataStoragePattern::List => {
                let list =
                    self.data_list
                    .as_ref()
                    .unwrap();
                for (k, v) in list {
                    println!("  {} : {},", k, v.as_ref().borrow());
                }
            },
            DataStoragePattern::Map => {
                let map =
                    self.data_map
                    .as_ref()
                    .unwrap();

                for (k, v) in map {
                    println!("  {} : {},", k, v.as_ref().borrow());
                }
            },
        }
        write!(f, "}}")
    }
}