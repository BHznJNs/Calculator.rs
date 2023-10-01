use std::rc::Rc;

use super::Value;

#[derive(PartialEq, Clone)]
pub enum VoidSign {
    Continue,
    Break(Rc<Value>),
    Empty,
}