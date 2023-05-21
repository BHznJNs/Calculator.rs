use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::public::colored_output::{NUMBER_COLORED, BOOLEAN_COLORED, INTERNAL_COLORED, STRING_COLORED};

use super::array::{ArrayLiteral, self};
use super::number::Number;
use super::super::compile_time::ast::ast_enum::ASTNode;
use super::function::{UserDefinedFunction, BuildInFunction, Function, Overload as FunctionOverload};
use super::oop::class::Class;
use super::oop::object::{Object, self};

#[derive(PartialEq, Clone, Copy)]
pub enum ValueType {
    Void, // all value type

    Boolean,
    Number,
    String,
    Array,
    LazyExpression,

    Function,
    Class,
    Object,
}

pub const VALUE_TYPE_PAIRS: [(&'static str, ValueType); 8] = [
    ("_"        , ValueType::Void),

    ("Bool"     , ValueType::Boolean),
    ("Numb"     , ValueType::Number),
    ("Str"      , ValueType::String),
    ("Array"    , ValueType::Array),

    ("LazyExpr" , ValueType::LazyExpression),
    ("Func"     , ValueType::Function),
    ("Obj"      , ValueType::Object),
];

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Void           => write!(f, "Void"),
            ValueType::Boolean        => write!(f, "Boolean"),
            ValueType::Number         => write!(f, "Number"),
            ValueType::String         => write!(f, "String"),
            ValueType::Array          => write!(f, "Array"),
            ValueType::LazyExpression => write!(f, "LazyExpression"),
            ValueType::Function       => write!(f, "Function"),
            ValueType::Class          => write!(f, "Class"),
            ValueType::Object         => write!(f, "Object"),
        }
    }
}

// --- --- --- --- --- ---
#[derive(PartialEq, Clone)]
pub enum VoidSign {
    Continue,
    Break(Rc<Value>),
    Empty,
}
#[derive(PartialEq, Clone)]
pub enum Value {
    // Value::Void(None)
    // is used when comment line
    // or blank line or
    // or return state for statement.

    Void(VoidSign),

    Boolean(bool),
    Number(Number),
    String(Rc<RefCell<String>>),
    Array(Rc<RefCell<ArrayLiteral>>),
    LazyExpression(Rc<ASTNode>),

    Function(Function),
    Class(Rc<Class>),
    Object(Rc<RefCell<Object>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void(option_val) =>
                match option_val {
                    VoidSign::Continue => write!(f, "Void(Continue)"),
                    VoidSign::Break(val) => write!(f, "Void({})", val),
                    VoidSign::Empty => write!(f, "<Void>"),
                },

            Value::Boolean(bool_val) =>
                write!(f, "{}", BOOLEAN_COLORED.output(bool_val)),
            Value::Number(num) =>
                write!(f, "{}", NUMBER_COLORED.output(num)),
            Value::String(str) =>
                write!(f, "{}", str.as_ref().borrow()),
            Value::LazyExpression(_) =>
                write!(f, "{}", INTERNAL_COLORED.output("<Lazy-Expression>")),
            Value::Function(func) =>
                write!(f, "{}", INTERNAL_COLORED.output(func)),
            Value::Array(arr) =>
                Ok(array::display(arr.clone(), 1)),

            Value::Class(cls) =>
                write!(f, "{}", cls),
            Value::Object(obj) =>
                Ok(object::display(obj.clone(), 1)),
        }
    }
}

impl Value {
    // formater for string typed value
    pub fn str_format(&self) -> String {
        STRING_COLORED.output(format!("\"{}\"", self))
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

    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Void(_)           => ValueType::Void,

            Value::Boolean(_)        => ValueType::Boolean,
            Value::Number(_)         => ValueType::Number,
            Value::String(_)         => ValueType::String,
            Value::Array(_)          => ValueType::Array,
            Value::LazyExpression(_) => ValueType::LazyExpression,

            Value::Function(_)       => ValueType::Function,
            Value::Class(_)          => ValueType::Class,
            Value::Object(_)         => ValueType::Object,
        }
    }
    pub fn check_type(&self, target_type: ValueType) -> bool {
        if target_type == ValueType::Void {
            // `void` type can be any type
            return true
        }

        return self.get_type() == target_type
    }
}

// Overload functions
pub trait Overload<T> {
    fn create(value: T) -> Self;
}

impl Overload<bool> for Value {
    fn create(value: bool) -> Self {
        Value::Boolean(value)
    }
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
        Value::Function(Function::create(value))
    }
}
impl Overload<BuildInFunction> for Value {
    fn create(value: BuildInFunction) -> Self {
        Value::Function(Function::create(value))
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