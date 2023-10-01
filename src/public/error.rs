use core::fmt;
use std::fmt::Display;

use crossterm::style::{StyledContent, Stylize};

use crate::utils::print_line;

use super::value::value::ValueType;

type ErrorResult = Result<(), ()>;
fn error_name_output(name: &str) -> StyledContent<&str> {
    name.white().on_red().bold()
}

const TYPE_ERROR_NAME: &str = " TypeError ";
pub fn type_error(param: Option<&str>, expected: Vec<ValueType>, found: ValueType) -> ErrorResult {
    // Vec<ValueType> -> "{type}/{type} ..."
    fn join(mut type_vec: Vec<ValueType>) -> String {
        let mut res_string = String::new();
        loop {
            let current = type_vec.remove(0);
            res_string.extend(current.to_string().chars());

            if type_vec.len() != 0 {
                res_string.push('/');
            } else {
                break;
            }
        }
        return res_string;
    }

    print!("{}", error_name_output(TYPE_ERROR_NAME));
    if let Some(name) = param {
        print!(" for \"{}\"", name);
    }
    print_line(format!(": expected {}, found {}.", join(expected), found));
    return Err(());
}

const MATH_ERROR_NAME: &str = " MathError ";
pub fn math_error(msg: &str) -> ErrorResult {
    print_line(format!("{}: {}.", error_name_output(MATH_ERROR_NAME), msg));
    return Err(());
}

const RANGE_ERROR_NAME: &str = " RangeError ";
pub fn range_error<T: Display>(param: &str, expected: T, found: usize) -> ErrorResult {
    print!("{} for \"{}\"", error_name_output(RANGE_ERROR_NAME), param);
    print_line(format!(": expected {}, found {}.", expected, found));
    return Err(());
}

const SYNTAX_ERROR_NAME: &str = " SyntaxError ";
pub fn syntax_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.\r",
        error_name_output(SYNTAX_ERROR_NAME),
        msg
    ));
    return Err(());
}

const ASSIGNMENT_ERROR_NAME: &str = " SyntaxError ";
pub fn assignment_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.",
        error_name_output(ASSIGNMENT_ERROR_NAME),
        msg
    ));
    return Err(());
}

const REFERENCE_ERROR_NAME: &str = " ReferenceError ";
pub enum ReferenceType {
    Variable,
    Property,
}
pub fn reference_error(type__: ReferenceType, target_name: &str) -> ErrorResult {
    print_line(format!(
        "{}: {} `{}` is not defined.",
        error_name_output(REFERENCE_ERROR_NAME),
        match type__ {
            ReferenceType::Variable => "variable",
            ReferenceType::Property => "property",
        },
        target_name,
    ));
    return Err(());
}

const IMPORT_ERROR_NAME: &str = " ImportError ";
pub fn import_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.",
        error_name_output(IMPORT_ERROR_NAME),
        msg
    ));
    return Err(());
}

// --- --- --- --- --- ---

pub enum InternalComponent {
    Std,
    InternalFn,

    Tokenizer,
    Analyzer,
    Computer,
}
impl fmt::Display for InternalComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalComponent::Std => write!(f, "Standard-Library"),
            InternalComponent::InternalFn => write!(f, "Internal-Function"),

            InternalComponent::Tokenizer => write!(f, "Tokenizer"),
            InternalComponent::Analyzer => write!(f, "Analyzer"),
            InternalComponent::Computer => write!(f, "Computer"),
        }
    }
}

const INTERNAL_ERROR_NAME: &str = " InternalError ";
pub fn internal_error(from: InternalComponent, msg: &str) -> ErrorResult {
    print_line(format!(
        "{} from {}: {}.",
        error_name_output(INTERNAL_ERROR_NAME),
        from,
        msg
    ));
    return Err(());
}

