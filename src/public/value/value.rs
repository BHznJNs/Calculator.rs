use std::fmt;

use super::number::Number;
use super::super::compile_time::ast::ASTNode;

pub enum ValueTypes {
    Number,
    Array,
    String,
    LazyExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(Number),
    Array(Vec<Value>),
    LazyExpression(ASTNode),
}
pub type ValueVec = Vec<Value>;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::LazyExpression(le) => write!(f, "{}", le),
        }
    }
}