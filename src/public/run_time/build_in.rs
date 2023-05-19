use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::consts::E;
// use std::f64::EPSILON;

use crate::public::value::number::Number;
use crate::public::value::value::Overload;
use crate::public::value::value::Value;
use crate::public::value::value::ValueType;

#[derive(PartialEq, Clone)]
pub enum BuildInFnEnum {
    // Basic
    Input, Type, Len,
    Int, Float, String, Array, Ascii,

    // Math
     Sin,  Cos,  Tan,
    Asin, Acos, Atan,
    Sinh, Cosh, Tanh,

    Rad, Deg,

    Log10, Log2, Log, Ln, Exp,

    Abs, Sqrt, Floor, Round,

    // Array
    Push, Pop,
    Shift, Unshift,
    Insert, Remove,
    Clone,

    // String
    Split, Replace, Repeat, Join,
    StartWith, EndWith,

    // // File system
    // Read, Write
}

pub fn constants() -> HashMap<String, Value> {
    HashMap::from([
        (String::from("VOID")     , Value::create(ValueType::Void   as i64)),
        (String::from("NUMBER")   , Value::create(ValueType::Number as i64)),
        (String::from("STRING")   , Value::create(ValueType::String as i64)),
        (String::from("ARRAY")    , Value::create(ValueType::Array  as i64)),
        (String::from("LAZYEXPR") , Value::create(ValueType::LazyExpression as i64)),
        (String::from("FUNCTION") , Value::create(ValueType::Function as i64)),
        (String::from("CLASS")    , Value::create(ValueType::Class  as i64)),
        (String::from("OBJECT")   , Value::create(ValueType::Object as i64)),

        (String::from("EPS")      , Value::Number(Number::Float(f64::EPSILON))),
        (String::from("PI")       , Value::Number(Number::Float(PI))),
        (String::from("E")        , Value::Number(Number::Float(E ))),

        (String::from("TRUE")     , Value::Boolean(true)),
        (String::from("FALSE")    , Value::Boolean(false)),
    ])
}