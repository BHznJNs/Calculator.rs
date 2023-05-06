use std::rc::Rc;

use crate::public::compile_time::ast::ASTNodeVec;
use crate::public::run_time::build_in::BuildInFnEnum;
use crate::public::std::std::StdModules;

use super::value::ValueTypes;

#[derive(PartialEq)]
pub struct BuildInParam {
    pub type__: ValueTypes,
    pub identi: &'static str,
}

#[derive(PartialEq)]
pub struct BuildInFunction {
    pub params: [Option<BuildInParam>; 4],
    pub lib: StdModules,
    pub body: BuildInFnEnum,
}

#[derive(PartialEq, Clone)]
pub struct Param {
    pub type__: ValueTypes,
    pub identi: String,
}

#[derive(PartialEq)]
pub struct UserDefinedFunction {
    pub params: Vec<Param>,
    pub body: ASTNodeVec,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Function {
    BuildIn(Rc<BuildInFunction>),
    UserDefined(Rc<UserDefinedFunction>),
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