use std::{cell::RefCell, collections::{VecDeque, vec_deque::Iter}, fmt, rc::Rc, ops::{Index, IndexMut}};

use super::value::Value;
use crate::public::value::{display_indent, oop::object};
use crossterm::style::Stylize;

pub type ArrayLiteral = VecDeque<Value>;

pub struct RawArray(pub(self) ArrayLiteral);

impl RawArray {
    pub fn new(value: ArrayLiteral) -> Self {
        return Self(value);
    }
    
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.0.push_back(value);
    }
    #[inline]
    pub fn pop(&mut self) -> Option<Value> {
        return self.0.pop_back();
    }
    #[inline]
    pub fn unshift(&mut self, value: Value) {
        self.0.push_front(value);
    }
    #[inline]
    pub fn shift(&mut self) -> Option<Value> {
        return self.0.pop_front();
    }
    #[inline]
    pub fn insert(&mut self, index: usize, value: Value) {
        self.0.insert(index, value);
    }
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<Value> {
        return self.0.remove(index);
    }


    pub fn join(&self, div: &str) -> String {
        if self.0.is_empty() {
            return String::new();
        }
    
        let mut arr_iter = self.0.iter();
        let mut result_str = arr_iter.next().unwrap().to_raw_string();
    
        for v in arr_iter {
            result_str.extend(div.chars());
            result_str.extend(v.to_raw_string().chars());
        }
        return result_str;
    }

    #[inline]
    pub fn iter(&self) -> Iter<Value> {
        return self.0.iter();
    }
    #[inline]
    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

impl Index<usize> for RawArray {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}
impl IndexMut<usize> for RawArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

// recursively clone array elements
pub fn deep_clone(arr: &Rc<RefCell<RawArray>>) -> Value {
    let mut new_array = ArrayLiteral::new();

    for v in arr.borrow().iter() {
        let element = if let Value::Array(arr) = v {
            self::deep_clone(arr)
        } else {
            v.deep_clone()
        };
        new_array.push_back(element);
    }
    return Value::from(new_array);
}

pub fn display(
    f: &mut fmt::Formatter<'_>,
    arr: &Rc<RefCell<RawArray>>,
    level: usize,
) -> fmt::Result {
    const LINE_COUNT: i8 = 5;
    let mut index = 0;

    write!(f, "[")?;
    for element in arr.borrow().iter() {
        // print indent
        if index % LINE_COUNT == 0 {
            write!(f, "\r\n")?;
            write!(f, "{}", "  ".repeat(level))?;
        }

        // print elements
        match element {
            Value::String(_) => write!(f, "{}", element.str_format())?,
            Value::Array(arr) => self::display(f, arr, level + 1)?,
            Value::Object(obj) => object::display(f, obj, level + 1)?,
            _ => write!(f, "{}", element)?,
        }
        write!(f, "{}", ", ".dim())?;
        index += 1;
    }

    write!(f, "\r\n")?;
    write!(f, "{}]", display_indent(level - 1))
}
