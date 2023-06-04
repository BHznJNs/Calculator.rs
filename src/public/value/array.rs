use std::{cell::RefCell, collections::VecDeque, rc::Rc, io};

use crate::{public::value::oop::object, utils::output::print_line};

use super::value::{Value, Overload};

pub type ArrayLiteral = VecDeque<Value>;

// recursively clone array elements
pub fn deep_clone(arr: Rc<RefCell<ArrayLiteral>>) -> Value {
    let mut new_array = ArrayLiteral::new();

    for i in &*(arr.as_ref().borrow()) {
        let element =
        if let Value::Array(arr) = i {
            deep_clone(arr.clone())
        } else {
            i.deep_clone()
        };
        new_array.push_back(element);
    }
    return Value::create(new_array);
}

pub fn display(arr: Rc<RefCell<ArrayLiteral>>, level: usize) {
    const LINE_COUNT: i8 = 5;
    let mut index = 0;
    let mut stdout = io::stdout();

    print!("[");
    let iterator = &*(arr.as_ref().borrow());
    for element in iterator {
        // print indent
        if index % LINE_COUNT == 0 {
            print_line(&mut stdout, "");
            print!("{}", "  ".repeat(level));
        }

        // print elements
        match element {
            Value::String(_) =>
                print!("{}", element.str_format()),
            Value::Array(arr) =>
                display(arr.clone(), level + 1),
            Value::Object(obj) =>
                object::display(obj.clone(), level + 1),
            _ => print!("{}", element),
        }

        print!(", ");
        index += 1;
    }

    print_line(&mut stdout, "");
    print!("{}]", "  ".repeat(level - 1))
}
