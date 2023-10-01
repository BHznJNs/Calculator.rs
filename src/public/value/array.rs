use std::{
    cell::RefCell,
    collections::{vec_deque::Iter, VecDeque},
    fmt,
    ops::{Index, IndexMut},
    rc::Rc,
};

use super::{core::Value, ComplexStructure, GetAddr};
use crate::public::value::display_indent;
use crossterm::style::Stylize;

pub type ArrayLiteral = VecDeque<Value>;

pub struct RawArray(pub(self) ArrayLiteral);

impl RawArray {
    pub fn new() -> Self {
        return Self(ArrayLiteral::new());
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

    #[inline]
    pub fn contains(&self, value: &Value) -> bool {
        return self.0.contains(value);
    }

    pub fn slice(&self, start: i64, mut end: i64) -> Self {
        let self_len = self.len() as i64;
        if start >= self_len {
            Self::new()
        } else {
            // size = len - 1
            // end > size || end == 0: get value until size
            // end < size && end > 0: normal
            // end < 0: get value until (size + end)

            match end {
                x if x >= self_len || x == 0 => end = self_len,
                x if x < 0 => end += self_len,
                x if x < self_len => {}
                _ => unreachable!(),
            }

            let mut res_arr = ArrayLiteral::new();
            for index in start as usize..end as usize {
                let cloned = self.0[index].clone();
                res_arr.push_back(cloned);
            }

            Self::from(res_arr)
        }
    }

    pub fn join(&self, div: &str) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let mut arr_iter = self.0.iter();
        let mut result_str = arr_iter.next().unwrap().to_raw_string();

        for v in arr_iter {
            result_str.push_str(div);
            result_str.push_str(&v.to_raw_string());
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

impl From<ArrayLiteral> for RawArray {
    fn from(value: ArrayLiteral) -> Self {
        return Self(value);
    }
}

impl GetAddr for RawArray {
    fn get_addr(&self) -> super::Addr {
        let ptr = self as *const Self;
        return ptr as super::Addr;
    }
}

impl ComplexStructure for RawArray {
    fn display(f: &mut fmt::Formatter<'_>, arr: &Rc<RefCell<Self>>, level: usize) -> fmt::Result {
        const LINE_COUNT: usize = 5;

        write!(f, "[")?;
        for (index, element) in arr.borrow().iter().enumerate() {
            // print indent
            if index % LINE_COUNT == 0 {
                write!(f, "\r\n")?;
                write!(f, "{}", "  ".repeat(level))?;
            }
            // print element
            Self::item_display(f, element, level + 1)?;
            // comma symbol print
            write!(f, "{}", ", ".dim())?;
        }

        write!(f, "\r\n")?;
        write!(f, "{}]", display_indent(level - 1))
    }

    fn deep_clone(arr: &Rc<RefCell<Self>>) -> Value {
        let mut new_array = ArrayLiteral::new();

        for v in arr.borrow().iter() {
            let element = if let Value::Array(arr) = v {
                Self::deep_clone(arr)
            } else {
                v.deep_clone()
            };
            new_array.push_back(element);
        }
        return Value::from(new_array);
    }
}
