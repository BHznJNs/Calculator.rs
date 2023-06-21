use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::public::run_time::scope::GlobalScope;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::Value;

use super::object::Object;

// convert module scope to Object
pub fn module_create(module_props: GlobalScope) -> Object {
    let prop_count = module_props.variables.len();
    let storage_pattern = if prop_count > Class::STORAGE_THRESHOLD {
        DataStoragePattern::Map
    } else {
        DataStoragePattern::List
    };

    let data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>;
    let data_map: Option<HashMap<String, Rc<RefCell<Value>>>>;

    match storage_pattern {
        DataStoragePattern::List => {
            let mut list = Vec::<(String, Rc<RefCell<Value>>)>::new();
            for (k, v) in module_props.variables {
                list.push((k, Rc::new(RefCell::new(v))));
            }
            data_list = Some(list);
            data_map = None;
        }
        DataStoragePattern::Map => {
            let mut map = HashMap::<String, Rc<RefCell<Value>>>::new();
            for (k, v) in module_props.variables {
                map.insert(k, Rc::new(RefCell::new(v)));
            }
            data_list = None;
            data_map = Some(map);
        }
    }
    Object {
        prototype: None,
        storage_pattern,
        data_list,
        data_map,
    }
}
