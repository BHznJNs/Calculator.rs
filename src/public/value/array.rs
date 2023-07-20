use std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc};

use super::value::Value;
use crate::public::value::{display_indent, oop::object::Object};
use crossterm::style::Stylize;

pub type ArrayLiteral = VecDeque<Value>;

pub struct Array;

impl Array {
    // recursively clone array elements
    pub fn deep_clone(arr: Rc<RefCell<ArrayLiteral>>) -> Value {
        let mut new_array = ArrayLiteral::new();

        for i in &*(arr.as_ref().borrow()) {
            let element = if let Value::Array(arr) = i {
                Array::deep_clone(arr.clone())
            } else {
                i.deep_clone()
            };
            new_array.push_back(element);
        }
        return Value::from(new_array);
    }

    pub fn join(arr: &ArrayLiteral, div: &str) -> String {
        if arr.is_empty() {
            return String::new();
        }

        let mut arr_iter = arr.iter();
        let mut result_str = arr_iter.next().unwrap().to_raw_string();

        for v in arr_iter {
            result_str.extend(div.chars());
            result_str.extend(v.to_raw_string().chars());
        }
        return result_str;
    }

    pub fn display(
        f: &mut fmt::Formatter<'_>,
        arr: &Rc<RefCell<ArrayLiteral>>,
        level: usize,
    ) -> fmt::Result {
        const LINE_COUNT: i8 = 5;
        let mut index = 0;

        write!(f, "[")?;
        let iterator = &*(arr.as_ref().borrow());
        for element in iterator {
            // print indent
            if index % LINE_COUNT == 0 {
                write!(f, "\r\n")?;
                write!(f, "{}", "  ".repeat(level))?;
            }

            // print elements
            match element {
                Value::String(_) => write!(f, "{}", element.str_format())?,
                Value::Array(arr) => Array::display(f, arr, level + 1)?,
                Value::Object(obj) => Object::display(f, obj, level + 1)?,
                _ => write!(f, "{}", element)?,
            }
            write!(f, "{}", ", ".dim())?;
            index += 1;
        }

        write!(f, "\r\n")?;
        write!(f, "{}]", display_indent(level - 1))
    }
}
