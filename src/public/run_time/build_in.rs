use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::consts::E;
use std::rc::Rc;

use crate::public::value::number::Number;
use crate::public::value::value::Overload;
use crate::public::value::value::Value;
use crate::public::value::value::ValueTypes;

#[derive(Clone)]
pub enum BuildInFuncs {
    // Basic
    Type,
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
    Update, Len,

    // // File system
    // Read, Write
}

pub fn variables() -> HashMap<String, Rc<Value>> {
    HashMap::from([
        (String::from("VOID")  , Rc::new(Value::create(ValueTypes::Void   as i64))),
        (String::from("NUM")   , Rc::new(Value::create(ValueTypes::Number as i64))),
        (String::from("STR")   , Rc::new(Value::create(ValueTypes::String as i64))),
        (String::from("ARR")   , Rc::new(Value::create(ValueTypes::Array  as i64))),
        (String::from("LEXPR") , Rc::new(Value::create(ValueTypes::LazyExpression as i64))),
        (String::from("FUNC")  , Rc::new(Value::create(ValueTypes::Function as i64))),

        (String::from("PI"), Rc::new(Value::Number(Number::Float(PI)))),
        (String::from("E") , Rc::new(Value::Number(Number::Float(E )))),
    ])
}