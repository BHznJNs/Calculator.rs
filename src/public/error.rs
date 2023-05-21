use core::fmt;

use crate::public::colored_output::ERROR_COLORED;

use super::value::value::ValueType;

type ErrorResult = Result<(), ()>;

pub fn type_error(
    param: Option<&str>,
    expected: ValueType,
    found: ValueType,
) -> ErrorResult {
    print!("{}", ERROR_COLORED.output(" TypeError "));
    if let Some(name) = param {
        print!(" for \"{}\"", name);
    }
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
    Tokenizer,
    Analyzer,
    Computer,
}
impl fmt::Display for InternalComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalComponent::Tokenizer => write!(f, "tokenizer"),
            InternalComponent::Analyzer  => write!(f, "analyzer"),
            InternalComponent::Computer  => write!(f, "computer"),
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