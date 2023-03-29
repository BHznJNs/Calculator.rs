use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::consts::E;
use std::rc::Rc;

use crate::public::value::number::Number;
use crate::public::value::value::Value;

#[derive(Clone)]
pub enum BuildInFuncs {
    // Basic
    Int, Float, String,

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
        (String::from("PI"), Rc::new(Value::Number(Number::Float(PI)))),
        (String::from("E") , Rc::new(Value::Number(Number::Float(E )))),
    ])
}