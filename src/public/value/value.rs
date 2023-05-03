use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

use super::number::Number;
use super::super::compile_time::ast::ASTNode;
use super::function::UserDefinedFunction;
use super::oop::class::Class;
use super::oop::object::Object;

#[derive(PartialEq, Clone)]
pub enum ValueTypes {
    Void, // all value type

    Number,
    String,
    Array,
    LazyExpression,

    Function,
    Class,
    Object,
}
pub const VALUE_TYPE_ARR: [&'static str; 7] = [
    "_",
    "num",
    "str",
    "arr",
    "lexpr",

    "func",
    "obj",
];
pub const VALUE_TYPE_ENUM: [ValueTypes; 7] = [
    ValueTypes::Void,
    ValueTypes::Number,
    ValueTypes::String,
    ValueTypes::Array,
    ValueTypes::LazyExpression,

    ValueTypes::Function,
    ValueTypes::Object,
];

impl fmt::Display for ValueTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueTypes::Void   => write!(f, "Void"),
            ValueTypes::Number => write!(f, "Number"),
            ValueTypes::String => write!(f, "String"),
            ValueTypes::Array  => write!(f, "Array"),
            ValueTypes::LazyExpression => write!(f, "LazyExpression"),
            ValueTypes::Function => write!(f, "Function"),
            ValueTypes::Class  => write!(f, "Class"),
            ValueTypes::Object => write!(f, "Object"),
        }
    }
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Value {
    Void(Option<Rc<Value>>),

    Number(Number),
    String(Rc<RefCell<String>>),
    Array(Rc<RefCell<ArrayLiteral>>),
    LazyExpression(Rc<ASTNode>),

    Function(Rc<UserDefinedFunction>),
    Class(Rc<Class>),
    Object(Rc<RefCell<Object>>),
}
pub type ArrayLiteral = VecDeque<Value>;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void(_) => write!(f, "void"),

            Value::Number(num) => write!(f, "{}", num),
            Value::String(str) => write!(f, "{}", str.as_ref().borrow()),
            Value::LazyExpression(_) => write!(f, "<Lazy-Expression>"),
            Value::Function(_) => write!(f, "<User-Defined-Function>"),
            Value::Array(arr) => {
                const LINE_COUNT: i8 = 5;
                let mut index = 0;
                write!(f, "[")?;
                let iterator = &*(*arr.as_ref()).borrow();
                // for element in arr.as_ref() {
                for element in iterator {
                    if index % LINE_COUNT == 0 {
                        write!(f, "\n  ")?;
                    }
                    write!(f, "{}, ", element)?;
                    index += 1;
                }
                write!(f, "\n]")
            },
            Value::Class(cls) =>
                write!(f, "{}", cls),
            Value::Object(obj) =>
                write!(f, "{}", obj.as_ref().borrow()),
        }
    }
}

impl Value {
    pub fn empty() -> Value {
        Value::Number(Number::Empty)
    }
    pub fn get_i64(&self) -> Result<i64, ()> {
        let Value::Number(num) = self else {
            println!("Target value is not a valid number value.");
            return Err(())
        };
        Ok(num.int_value())
    }
    pub fn get_f64(&self) -> Result<f64, ()> {
        let Value::Number(num) = self else {
            println!("Target value is not a valid number value.");
            return Err(())
        };
        Ok(num.float_value())
    }

    pub fn unwrap(&self) -> Value {
        // Rc<Value> -> Value
        self.clone()
    }
    pub fn deep_clone(&self) -> Result<Value, ()> {
        match self {
            Value::Number(_) => Ok(self.clone()),
            Value::String(str) => {
                let cloned_str =
                    str.as_ref()
                    .borrow().clone();
                Ok(Value::create(cloned_str))
            },
            Value::Array(arr) => {
                let cloned_arr =
                    arr.as_ref()
                    .borrow().clone();
                Ok(Value::create(cloned_arr))
            },
            Value::Object(obj) => {
                let cloned_obj =
                    obj.as_ref()
                    .borrow().clone();
                Ok(Value::create(cloned_obj))
            },
            _ => {
                println!("Invalid clone type.");
                Err(())
            }
        }
    }


    pub fn get_type(&self) -> ValueTypes {
        match self {
            Value::Void(_)   => ValueTypes::Void,

            Value::Number(_) => ValueTypes::Number,
            Value::String(_) => ValueTypes::String,
            Value::Array(_)  => ValueTypes::Array,
            Value::LazyExpression(_) => ValueTypes::LazyExpression,

            Value::Function(_) => ValueTypes::Function,
            Value::Class(_)    => ValueTypes::Class,
            Value::Object(_)   => ValueTypes::Object,
        }
    }
    pub fn check_type(&self, target_type: &ValueTypes) -> bool {
        if *target_type == ValueTypes::Void {
            // `void` type can be any type
            return true
        }

        match self {
            Value::Void(_)   => *target_type == ValueTypes::Void,
            Value::Number(_) => *target_type == ValueTypes::Number,
            Value::String(_) => *target_type == ValueTypes::String,
            Value::Array(_)  => *target_type == ValueTypes::Array,
            Value::LazyExpression(_) => *target_type == ValueTypes::LazyExpression,

            Value::Function(_) => *target_type == ValueTypes::Function,
            Value::Class(_)    => *target_type == ValueTypes::Class,
            Value::Object(_)   => *target_type == ValueTypes::Object,
        }
    }
}

// Overload functions
pub trait Overload<T> {
    fn create(value: T) -> Self;
}

impl Overload<i64> for Value {
    fn create(value: i64) -> Self {
        Value::Number(Number::Int(value))
    }
}
impl Overload<f64> for Value {
    fn create(value: f64) -> Self {
        Value::Number(Number::Float(value))
    }
}
impl Overload<String> for Value {
    fn create(value: String) -> Self {
        Value::String(Rc::new(RefCell::new(value)))
    }
}
impl Overload<ArrayLiteral> for Value {
    fn create(value: ArrayLiteral) -> Self {
        Value::Array(Rc::new(RefCell::new(value)))
    }
}
impl Overload<ASTNode> for Value {
    fn create(value: ASTNode) -> Self {
        Value::LazyExpression(Rc::new(value))
    }
}

impl Overload<UserDefinedFunction> for Value {
    fn create(value: UserDefinedFunction) -> Self {
        Value::Function(Rc::new(value))
    }
}
impl Overload<Class> for Value {
    fn create(value: Class) -> Self {
        Value::Class(Rc::new(value))
    }
}
impl Overload<Object> for Value {
    fn create(value: Object) -> Self {
        Value::Object(Rc::new(RefCell::new(value)))
    }
}