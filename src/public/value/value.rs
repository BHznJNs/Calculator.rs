use super::number::Number;

pub enum Value {
    Number(Number),
    String(String),
    Array(Vec<Number>),
}