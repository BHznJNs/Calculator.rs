use core::fmt;
use std::fmt::Display;

use crate::public::colored_output::ERROR_COLORED;

use super::value::value::ValueType;

type ErrorResult = Result<(), ()>;

pub fn type_error(
    param: Option<&str>,
    expected: Vec<ValueType>,
    found: ValueType,
) -> ErrorResult {
    // Vec<ValueType> -> "{type}/{type} ..."
    fn join(mut type_vec: Vec<ValueType>) -> String {
        let mut res_string = String::new();
        loop {
            let current =
                type_vec.remove(0);
            res_string += &format!("{}", current);

            if type_vec.len() != 0 {
                res_string.push('/');
            } else {
                break;
            }
        }
        return res_string
    }

    print!("{}", ERROR_COLORED.output(" TypeError "));
    if let Some(name) = param {
        print!(" for \"{}\"", name);
    }
    println!(": expected {}, found {}.", join(expected), found);
    Err(())
}

pub fn range_error<T: Display>(
    param: &str,
    expected: T,
    found: usize,
) -> ErrorResult {
    print!("{} for \"{}\"", ERROR_COLORED.output(" RangeError "), param);
    println!(": expected {}, found {}.", expected, found);
    Err(())
}

pub fn syntax_error(msg: &str) -> ErrorResult {
    println!("{}: {}.", ERROR_COLORED.output(" SyntaxError "), msg);
    Err(())
}

pub fn assignment_error(msg: &str) -> ErrorResult {
    println!("{}: {}.", ERROR_COLORED.output(" AssignmentError "), msg);
    Err(())
}

pub fn reference_error(var_name: &str) -> ErrorResult {
    println!("{}: variable `{}` is not defined.", ERROR_COLORED.output(" ReferenceError "), var_name);
    Err(())
}

pub fn import_error(msg: &str) -> ErrorResult {
    println!("{}: {}.", ERROR_COLORED.output(" ImportError "), msg);
    Err(())
}

// --- --- --- --- --- ---

pub enum InternalComponent {
    Std,

    Tokenizer,
    Analyzer,
    Computer,
}
impl fmt::Display for InternalComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalComponent::Std => write!(f, "Standard-Library"),

            InternalComponent::Tokenizer => write!(f, "Tokenizer"),
            InternalComponent::Analyzer  => write!(f, "Analyzer"),
            InternalComponent::Computer  => write!(f, "Computer"),
        }
    }
}

pub fn internal_error(
    from: InternalComponent,
    msg: &str
) -> ErrorResult {
    println!("{} from {}: {}.", ERROR_COLORED.output(" InternalError "), from, msg);
    Err(())
}