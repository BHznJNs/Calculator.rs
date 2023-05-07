use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::consts::E;

use crate::public::value::number::Number;
use crate::public::value::value::Overload;
use crate::public::value::value::Value;
use crate::public::value::value::ValueTypes;

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
        (String::from("VOID")  , Value::create(ValueTypes::Void   as i64)),
        (String::from("NUM")   , Value::create(ValueTypes::Number as i64)),
        (String::from("STR")   , Value::create(ValueTypes::String as i64)),
        (String::from("ARR")   , Value::create(ValueTypes::Array  as i64)),
        (String::from("LEXPR") , Value::create(ValueTypes::LazyExpression as i64)),
        (String::from("FUNC")  , Value::create(ValueTypes::Function as i64)),
        (String::from("CLS")   , Value::create(ValueTypes::Class  as i64)),
        (String::from("OBJ")   , Value::create(ValueTypes::Object as i64)),

        (String::from("PI"), Value::Number(Number::Float(PI))),
        (String::from("E") , Value::Number(Number::Float(E ))),
    ])
}