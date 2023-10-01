use std::{collections::HashMap, fmt};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone, Copy)]
pub enum ValueType {
    Void, // all value type

    Boolean,
    Number,
    Unique,

    String,
    Array,
    Map,
    LazyExpression,

    Function,
    Class,
    Object,
}

static mut VALUE_TYPE_MAP: Option<HashMap<&str, ValueType>> = None;
impl ValueType {
    pub fn is_valid_type(identi: &str) -> Option<ValueType> {
        unsafe {
            if VALUE_TYPE_MAP.is_none() {
                VALUE_TYPE_MAP = Some(HashMap::from([
                    ("_", ValueType::Void),
                    ("any", ValueType::Void),
                    ("Any", ValueType::Void),
                    // --- --- --- --- --- ---
                    ("bool", ValueType::Boolean),
                    ("Bool", ValueType::Boolean),
                    ("boolean", ValueType::Boolean),
                    ("Boolean", ValueType::Boolean),
                    // --- --- --- --- --- ---
                    ("num", ValueType::Number),
                    ("Num", ValueType::Number),
                    ("numb", ValueType::Number),
                    ("Numb", ValueType::Number),
                    ("number", ValueType::Number),
                    ("Number", ValueType::Number),
                    // --- --- --- --- --- ---
                    ("uni", ValueType::Unique),
                    ("Uni", ValueType::Unique),
                    ("unique", ValueType::Unique),
                    ("Unique", ValueType::Unique),
                    // --- --- --- --- --- ---
                    ("str", ValueType::String),
                    ("Str", ValueType::String),
                    ("string", ValueType::String),
                    ("String", ValueType::String),
                    // --- --- --- --- --- ---
                    ("arr", ValueType::Array),
                    ("Arr", ValueType::Array),
                    ("array", ValueType::Array),
                    ("Array", ValueType::Array),
                    // --- --- --- --- --- ---
                    ("map", ValueType::Map),
                    ("Map", ValueType::Map),
                    // --- --- --- --- --- ---
                    ("lExpr", ValueType::LazyExpression),
                    ("LazyExpr", ValueType::LazyExpression),
                    // --- --- --- --- --- ---
                    ("Fn", ValueType::Function),
                    ("func", ValueType::Function),
                    ("Func", ValueType::Function),
                    ("function", ValueType::Function),
                    ("Function", ValueType::Function),
                    // --- --- --- --- --- ---
                    ("obj", ValueType::Object),
                    ("Obj", ValueType::Object),
                    ("object", ValueType::Object),
                    ("Object", ValueType::Object),
                    // --- --- --- --- --- ---
                    ("Cl", ValueType::Class),
                    ("class", ValueType::Class),
                    ("Class", ValueType::Class),
                ]))
            }
        };

        let target_type = unsafe {
            let Some(map) = &VALUE_TYPE_MAP else {
                unreachable!()
            };
            map.get(identi)
        };
        return target_type.copied();
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Void => write!(f, "Void"),
            ValueType::Boolean => write!(f, "Boolean"),
            ValueType::Number => write!(f, "Number"),
            ValueType::Unique => write!(f, "Unique"),
            ValueType::String => write!(f, "String"),
            ValueType::Array => write!(f, "Array"),
            ValueType::Map => write!(f, "Map"),
            ValueType::LazyExpression => write!(f, "LazyExpression"),
            ValueType::Function => write!(f, "Function"),
            ValueType::Class => write!(f, "Class"),
            ValueType::Object => write!(f, "Object"),
        }
    }
}
