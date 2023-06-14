use std::fmt;
use std::rc::Rc;

use crate::public::compile_time::ast::ast_enum::ASTVec;
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::error::{range_error, type_error};
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::{LocalScope, Scope};

use super::value::{ValueType, Value};

// #[derive(PartialEq, Clone)]
// pub struct Param {
//     pub type__: ValueType,
//     pub identi: &'static str,
// }

#[derive(PartialEq, Clone)]
pub struct BuildInFnParam(pub ValueType, pub &'static str);

impl Param for BuildInFnParam {
    fn type__(&self) -> ValueType {
        self.0
    }
    fn identi(&self) -> &str {
        self.1
    }
}

#[derive(PartialEq)]
pub struct BuildInFunction {
    pub params: Vec<BuildInFnParam>,
    pub identi: BuildInFnIdenti,
}

#[derive(PartialEq, Clone)]
pub struct UserDefinedFnParam {
    pub type__: ValueType,
    pub identi: String,
}
impl Param for UserDefinedFnParam {
    fn type__(&self) -> ValueType {
        self.type__
    }
    fn identi(&self) -> &str {
        &self.identi
    }
}
#[derive(PartialEq)]
pub struct UserDefinedFunction {
    pub params: Vec<UserDefinedFnParam>,
    pub body: ASTVec,
}

pub trait Param {
    fn type__(&self) -> ValueType;
    fn identi(&self) -> &str;
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Function {
    BuildIn(Rc<BuildInFunction>),
    UserDefined(Rc<UserDefinedFunction>),
}

impl Function {
    pub fn param_check(
        formal_params: &Vec<impl Param>,
        actual_params: &Vec<ExpressionNode>,
        whole_scope: &mut Scope,
        local_scope: &mut LocalScope,
        expr_resolver: fn(Rc<ExpressionNode>, &mut Scope) -> Result<Value, ()>,
    ) -> Result<(), ()> {
        if actual_params.len() < formal_params.len() {
            // if param missing
            return Err(range_error(
                "function invocation",
                formal_params.len(),
                actual_params.len(),
            )?);
        }

        let mut index = 0;
        while index < formal_params.len() {
            let formal_param = &formal_params[index];

            // compute actual_param_value
            let actual_param_node = (&actual_params[index]).clone();
            let actual_param_value = expr_resolver(actual_param_node.into(), whole_scope)?;

            // param type check
            if actual_param_value.check_type(formal_param.type__()) {
                local_scope
                    .variables
                    .insert(formal_param.identi().to_string(), actual_param_value);
            } else {
                type_error(
                    Some(&formal_param.identi()),
                    vec![formal_param.type__()],
                    actual_param_value.get_type(),
                )?
            }

            index += 1;
        }
        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::BuildIn(_) => write!(f, "<Build-in-Function>"),
            Function::UserDefined(_) => write!(f, "<User-Defined-Function>"),
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
