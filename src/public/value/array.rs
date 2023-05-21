use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::public::value::oop::object;

use super::value::Value;

pub type ArrayLiteral = VecDeque<Value>;

pub fn display(arr: Rc<RefCell<ArrayLiteral>>, level: usize) {
    const LINE_COUNT: i8 = 5;
    let mut index = 0;

    print!("[");
    let iterator = &*(arr.as_ref().borrow());
    for element in iterator {
        // print indent
        if index % LINE_COUNT == 0 {
            print!("\n{}", "  ".repeat(level));
        }

        // print elements
        match element {
            Value::String(_) => print!("{}", element.str_format()),
            Value::Array(arr) => display(arr.clone(), level + 1),
            Value::Object(obj) => object::display(obj.clone(), level + 1),
            _ => print!("{}", element),
        }

        print!(", ");
        index += 1;
    }

    print!("\n{}]", "  ".repeat(level - 1))
}
