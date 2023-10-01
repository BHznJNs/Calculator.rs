use std::collections::HashMap;
use std::f64::consts::E as STD_E;
use std::f64::consts::PI as STD_PI;

use crate::public::value::unique::EMPTY_GLOBAL_UNIQUE;
use crate::public::value::{number::Number, unique::GlobalUnique, Value};

pub const PI: Value = Value::Number(Number::Float(STD_PI));
pub const E: Value = Value::Number(Number::Float(STD_E));
pub const TRUE: Value = Value::Boolean(true);
pub const FALSE: Value = Value::Boolean(false);

pub static mut IS_INITED: bool = false;
pub static mut VOID_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut BOOL_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut NUMBER_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut UNIQUE_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut STRING_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut ARRAY_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut MAP_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut LAZYEXPR_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut FUNCTION_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut CLASS_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;
pub static mut OBJECT_T: GlobalUnique = EMPTY_GLOBAL_UNIQUE;

unsafe fn static_init() {
    IS_INITED = true;

    VOID_T.init("Void-Type");
    BOOL_T.init("Boolean-Type");
    NUMBER_T.init("Number-Type");
    UNIQUE_T.init("Number-Type");
    STRING_T.init("String-Type");
    ARRAY_T.init("Array-Type");
    MAP_T.init("Map-Type");
    LAZYEXPR_T.init("Lazy-Expression-Type");
    FUNCTION_T.init("Function-Type");
    CLASS_T.init("Class-Type");
    OBJECT_T.init("Object-Type");
}

pub unsafe fn entry() -> HashMap<String, Value> {
    if !IS_INITED {
        static_init();
    }

    HashMap::from([
        (String::from("VOID"), Value::from(VOID_T.unwrap())),
        (String::from("BOOLEAN"), Value::from(BOOL_T.unwrap())),
        (String::from("NUMBER"), Value::from(NUMBER_T.unwrap())),
        (String::from("UNIQUE"), Value::from(UNIQUE_T.unwrap())),
        (String::from("STRING"), Value::from(STRING_T.unwrap())),
        (String::from("ARRAY"), Value::from(ARRAY_T.unwrap())),
        (String::from("MAP"), Value::from(MAP_T.unwrap())),
        (String::from("LAZYEXPR"), Value::from(LAZYEXPR_T.unwrap())),
        (String::from("FUNCION"), Value::from(FUNCTION_T.unwrap())),
        (String::from("CLASS"), Value::from(CLASS_T.unwrap())),
        (String::from("OBJECT"), Value::from(OBJECT_T.unwrap())),
        // --- --- --- --- --- ---
        (String::from("PI"), PI),
        (String::from("E"), E),
        (String::from("true"), TRUE),
        (String::from("false"), FALSE),
    ])
}
