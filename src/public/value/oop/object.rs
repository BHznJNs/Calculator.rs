use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use crate::public::value::function::UserDefinedFunction;

use super::super::value::Value;
use super::class::Class;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq)]
pub struct Object {
    pub prototype: Rc<Class>,

    pub storage_pattern: DataStoragePattern,
    pub data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>,
    pub data_map : Option<HashMap<String, Rc<RefCell<Value>>>>,
}

impl Object {
    pub fn get_method(
        &self, method_name: &String
    ) -> Result<Rc<UserDefinedFunction>, ()>  {
        self.prototype.get_method(method_name)
    }

    pub fn get(&self, prop_name: &String) -> Result<Rc<Value>, ()> {
        let target_value =
        getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        )?;
        let target_ref =
            target_value.as_ref().borrow();
        Ok(target_ref.get_ref())
    }

    pub fn set(
        &self,
        prop_name: &String,
        value: Value
    ) -> Result<(), ()> {
        let target_value =
        getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        )?;
        let mut target_ref =
            target_value.as_ref().borrow_mut();
        *target_ref = value;
        Ok(())
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