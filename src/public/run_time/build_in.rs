use std::collections::HashMap;
use std::f64::consts::E;
use std::f64::consts::PI;

use crate::public::std::modules::array::ArrayFn;
use crate::public::std::modules::basic::BasicFn;
use crate::public::std::modules::bit_ops::BitOpsFn;
use crate::public::std::modules::file_system::FileSysFn;
use crate::public::std::modules::math::MathFn;
use crate::public::std::modules::string::StringFn;

use crate::public::value::value::{Value, ValueType};

#[derive(PartialEq, Clone)]
pub enum BuildInFnIdenti {
    Basic(BasicFn),
    Math(MathFn),
    Array(ArrayFn),
    String(StringFn),
    FileSystem(FileSysFn),
    BitOps(BitOpsFn),
}

pub fn constants() -> HashMap<String, Value> {
    HashMap::from([
        (String::from("VOID"), Value::from(ValueType::Void as i64)),
        (
            String::from("BOOLEAN"),
            Value::from(ValueType::Boolean as i64),
        ),
        (
            String::from("NUMBER"),
            Value::from(ValueType::Number as i64),
        ),
        (
            String::from("STRING"),
            Value::from(ValueType::String as i64),
        ),
        (String::from("ARRAY"), Value::from(ValueType::Array as i64)),
        (
            String::from("LAZYEXPR"),
            Value::from(ValueType::LazyExpression as i64),
        ),
        (
            String::from("FUNCION"),
            Value::from(ValueType::Function as i64),
        ),
        (String::from("CLASS"), Value::from(ValueType::Class as i64)),
        (
            String::from("OBJECT"),
            Value::from(ValueType::Object as i64),
        ),
        (String::from("PI"), Value::from(PI)),
        (String::from("E"), Value::from(E)),
        (String::from("true"), Value::Boolean(true)),
        (String::from("false"), Value::Boolean(false)),
    ])
}
