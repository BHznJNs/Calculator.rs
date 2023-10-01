use std::{cell::RefCell, fmt, rc::Rc};

use self::{array::RawArray, map::RawMap, oop::object::Object};

pub mod array;
pub mod function;
pub mod map;
pub mod number;
pub mod symbols;
pub mod unique;

pub mod oop;
mod core;

pub use core::{Value, ValueType, VoidSign};

pub(super) type Addr = usize;
pub(super) trait GetAddr {
    fn get_addr(&self) -> Addr;
}

// --- --- --- --- --- ---

pub trait ComplexStructure {
    fn display(
        f: &mut fmt::Formatter<'_>,
        self_val: &Rc<RefCell<Self>>,
        level: usize,
    ) -> fmt::Result;
    fn deep_clone(self_val: &Rc<RefCell<Self>>) -> Value;

    fn item_display(f: &mut fmt::Formatter, value: &Value, level: usize) -> fmt::Result {
        match value {
            Value::String(_) => write!(f, "{}", value.str_format().unwrap()),
            Value::Array(arr) => RawArray::display(f, arr, level),
            Value::Map(map) => RawMap::display(f, map, level),
            Value::Object(obj) => Object::display(f, obj, level),
            _ => write!(f, "{}", value),
        }
    }
    fn item_clone(value: &Value) -> Value {
        match value {
            Value::String(str) => str.borrow().clone().into(),
            Value::Array(arr) => RawArray::deep_clone(arr),
            Value::Map(map) => RawMap::deep_clone(map),
            Value::Object(obj) => Object::deep_clone(obj),
            _ => value.deep_clone(),
        }
    }
}

#[inline]
fn display_indent(level: usize) -> String {
    "  ".repeat(level)
}

#[inline]
pub fn into_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    return Rc::new(RefCell::new(value));
}
