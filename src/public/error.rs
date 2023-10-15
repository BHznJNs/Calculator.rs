use core::fmt;
use std::fmt::Display;

use crossterm::style::{StyledContent, Stylize};

use super::value::ValueType;

pub type CalcError = String;
pub type CalcResult<T> = Result<T, CalcError>;

fn error_name_output(name: &str) -> StyledContent<&str> {
    name.white().on_red().bold()
}

pub fn type_error(param: Option<&str>, expected: Vec<ValueType>, found: ValueType) -> CalcError {
    const TYPE_ERROR_NAME: &str = " TypeError ";

    // Vec<ValueType> -> "{type}/{type} ..."
    fn join(mut type_vec: Vec<ValueType>) -> String {
        let mut res_string = String::new();
        loop {
            let current = type_vec.remove(0);
            res_string.push_str(&current.to_string());

            if !type_vec.is_empty() {
                res_string.push('/');
            } else {
                break;
            }
        }
        return res_string;
    }

    let mut err = CalcError::new();
    err.push_str(&error_name_output(TYPE_ERROR_NAME).to_string());
    if let Some(name) = param {
        err.push_str(&format!(" for \"{}\"", name));
    }
    err.push_str(&format!(": expected {}, found {}.", join(expected), found));
    return err;
}

pub fn math_error(msg: &str) -> CalcError {
    const MATH_ERROR_NAME: &str = " MathError ";
    return format!("{}: {}.", error_name_output(MATH_ERROR_NAME), msg);
}

pub fn range_error<T: Display>(param: &str, expected: T, found: usize) -> CalcError {
    const RANGE_ERROR_NAME: &str = " RangeError ";

    let mut err = CalcError::new();
    err.push_str(&format!("{} for \"{}\"", error_name_output(RANGE_ERROR_NAME), param));
    err.push_str(&format!(": expected {}, found {}.", expected, found));
    return err;
}

pub fn syntax_error(msg: &str) -> CalcError {
    const SYNTAX_ERROR_NAME: &str = " SyntaxError ";
    return format!(
        "{}: {}.",
        error_name_output(SYNTAX_ERROR_NAME),
        msg
    );
}

pub fn assignment_error(msg: &str) -> CalcError {
    const ASSIGNMENT_ERROR_NAME: &str = " SyntaxError ";
    return format!(
        "{}: {}.",
        error_name_output(ASSIGNMENT_ERROR_NAME),
        msg
    );
}

pub enum ReferenceType {
    Variable,
    Property,
}
pub fn reference_error(type__: ReferenceType, target_name: &str) -> CalcError {
    const REFERENCE_ERROR_NAME: &str = " ReferenceError ";
    return format!(
        "{}: {} `{}` is not defined.",
        error_name_output(REFERENCE_ERROR_NAME),
        match type__ {
            ReferenceType::Variable => "variable",
            ReferenceType::Property => "property",
        },
        target_name,
    );
}

pub fn import_error(msg: &str) -> CalcError {
    const IMPORT_ERROR_NAME: &str = " ImportError ";
    return format!(
        "{}: {}.",
        error_name_output(IMPORT_ERROR_NAME),
        msg
    );
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

pub fn internal_error(from: InternalComponent, msg: &str) -> CalcError {
    const INTERNAL_ERROR_NAME: &str = " InternalError ";
    return format!(
        "{} from {}: {}.",
        error_name_output(INTERNAL_ERROR_NAME),
        from,
        msg
    );
}
