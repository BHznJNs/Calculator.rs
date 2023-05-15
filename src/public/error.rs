use core::fmt;

use super::value::value::ValueType;

type ErrorResult = Result<(), ()>;

pub fn type_error(
    param: Option<&str>,
    expected: ValueType,
    found: ValueType,
) -> ErrorResult {
    print!("TypeError");
    if let Some(name) = param {
        print!(" for \"{}\"", name);
    }
    println!(": expected {}, found {}.", expected, found);
    Err(())
}

pub fn syntax_error(msg: &str) -> ErrorResult {
    println!("SyntaxError: {}.", msg);
    Err(())
}

pub fn assignment_error(msg: &str) -> ErrorResult {
    println!("SyntaxError: {}.", msg);
    Err(())
}

pub fn reference_error(var_name: &str) -> ErrorResult {
    println!("ReferenceError: variable `{}` is not defined.", var_name);
    Err(())
}

pub fn import_error(msg: &str) -> ErrorResult {
    println!("ImportError: {}.", msg);
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
    println!("InternalError from {}: {}.", from, msg);
    Err(())
}