use std::rc::Rc;
use std::fmt;

use crate::public::compile_time::ast::ast_enum::ASTVec;
use crate::public::run_time::build_in::BuildInFnIdenti;

use super::value::ValueType;

#[derive(PartialEq)]
pub struct BuildInParam {
    pub type__: ValueType,
    pub identi: &'static str,
}

#[derive(PartialEq)]
pub struct BuildInFunction {
    pub params: [Option<BuildInParam>; 4],
    pub identi: BuildInFnIdenti,
}

#[derive(PartialEq, Clone)]
pub struct Param {
    pub type__: ValueType,
    pub identi: String,
}

#[derive(PartialEq)]
pub struct UserDefinedFunction {
    pub params: Vec<Param>,
    pub body: ASTVec,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Function {
    BuildIn(Rc<BuildInFunction>),
    UserDefined(Rc<UserDefinedFunction>),
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::BuildIn(_) =>
                write!(f, "<Build-in-Function>"),
            Function::UserDefined(_) =>
                write!(f, "<User-Defined-Function>"),
        }
    }
}

pub trait Overload<T> {
    fn create(value: T) -> Self;
}

impl Overload<UserDefinedFunction> for Function {
    fn create(value: UserDefinedFunction) -> Self {
        Function::UserDefined(Rc::new(value))
    }
}
impl Overload<BuildInFunction> for Function {
    fn create(value: BuildInFunction) -> Self {
        Function::BuildIn(Rc::new(value))
    }
}