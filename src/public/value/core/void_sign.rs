use super::Value;

#[derive(PartialEq, Clone)]
pub enum VoidSign {
    Continue,
    Break(Box<Value>),
    Empty,
}