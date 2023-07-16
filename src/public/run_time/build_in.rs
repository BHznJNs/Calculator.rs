use std::collections::HashMap;
use std::f64::consts::E;
use std::f64::consts::PI;

use crate::public::std::modules::array::ArrayFn;
use crate::public::std::modules::basic::BasicFn;
use crate::public::std::modules::bit_ops::BitOpsFn;
use crate::public::std::modules::file_system::FileSysFn;
use crate::public::std::modules::math::MathFn;
use crate::public::std::modules::string::StringFn;

use crate::public::value::number::Number;
use crate::public::value::value::Overload;
use crate::public::value::value::Value;
use crate::public::value::value::ValueType;

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
        (String::from("VOID"), Value::create(ValueType::Void as i64)),
        (
            String::from("BOOLEAN"),
            Value::create(ValueType::Boolean as i64),
        ),
        (
            String::from("NUMBER"),
            Value::create(ValueType::Number as i64),
        ),
        (
            String::from("STRING"),
            Value::create(ValueType::String as i64),
        ),
        (
            String::from("ARRAY"),
            Value::create(ValueType::Array as i64),
        ),
        (
            String::from("LAZYEXPR"),
            Value::create(ValueType::LazyExpression as i64),
        ),
        (
            String::from("FUNCION"),
            Value::create(ValueType::Function as i64),
        ),
        (
            String::from("CLASS"),
            Value::create(ValueType::Class as i64),
        ),
        (
            String::from("OBJECT"),
            Value::create(ValueType::Object as i64),
        ),
        (String::from("PI"), Value::Number(Number::Float(PI))),
        (String::from("E"), Value::Number(Number::Float(E))),
        (String::from("true"), Value::Boolean(true)),
        (String::from("false"), Value::Boolean(false)),
    ])
}
