use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::public::value::{display_indent, ComplexStructure};
use std::collections::hash_map::{Iter, Keys, Values};

use super::core::Value;

pub type InternalMap = HashMap<String, Value>;
pub struct RawMap(pub(self) InternalMap);

impl RawMap {
    #[inline]
    pub fn new(map: InternalMap) -> Self {
        Self(map)
    }

    #[inline]
    pub fn get(&self, k: &str) -> Option<Value> {
        self.0.get(k).cloned()
    }
    #[inline]
    pub fn set(&mut self, k: String, v: Value) {
        self.0.insert(k, v);
    }

    #[inline]
    pub fn iter(&self) -> Iter<String, Value> {
        self.0.iter()
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, Value> {
        self.0.keys()
    }
    #[inline]
    pub fn values(&self) -> Values<String, Value> {
        self.0.values()
    }
    #[inline]
    pub fn has_key(&self, key_name: &str) -> bool {
        self.0.get(key_name).is_some()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ComplexStructure for RawMap {
    fn display(f: &mut fmt::Formatter<'_>, map: &Rc<RefCell<Self>>, level: usize) -> fmt::Result {
        write!(f, "{{\r\n")?;
        for (k, v) in map.borrow().iter() {
            write!(f, "{}{}: ", display_indent(level), k)?; // indent & key
            Self::item_display(f, v, level + 1)?; // value
            write!(f, "\r\n")?;
        }
        write!(f, "{}}}", display_indent(level - 1))
    }

    fn deep_clone(self_val: &Rc<RefCell<Self>>) -> Value {
        let mut new_map = InternalMap::new();
        for (k, v) in self_val.borrow().iter() {
            new_map.insert(k.to_owned(), Self::item_clone(v));
        }
        return Value::from(Self::new(new_map));
    }
}
