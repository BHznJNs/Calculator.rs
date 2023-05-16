use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::array::{ArrayLiteral, self};
use super::number::Number;
use super::super::compile_time::ast::ast_enum::ASTNode;
use super::function::{UserDefinedFunction, BuildInFunction, Function, Overload as FunctionOverload};
use super::oop::class::Class;
use super::oop::object::{Object, self};

#[derive(PartialEq, Clone, Copy)]
pub enum ValueType {
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
    "Number",
    "String",
    "Array",
    "LazyExpr",

    "Function",
    "Object",
];
pub const VALUE_TYPE_ENUM: [ValueType; 7] = [
    ValueType::Void,
    ValueType::Number,
    ValueType::String,
    ValueType::Array,
    ValueType::LazyExpression,

    ValueType::Function,
    ValueType::Object,
];

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Void   => write!(f, "Void"),
            ValueType::Number => write!(f, "Number"),
            ValueType::String => write!(f, "String"),
            ValueType::Array  => write!(f, "Array"),
            ValueType::LazyExpression => write!(f, "LazyExpression"),
            ValueType::Function => write!(f, "Function"),
            ValueType::Class  => write!(f, "Class"),
            ValueType::Object => write!(f, "Object"),
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

    Function(Function),
    Class(Rc<Class>),
    Object(Rc<RefCell<Object>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void(_) => write!(f, "void"),

            Value::Number(num) =>
                write!(f, "{}", num),
            Value::String(str) =>
                write!(f, "{}", str.as_ref().borrow()),
            Value::LazyExpression(_) =>
                write!(f, "<Lazy-Expression>"),
            Value::Function(func) =>
                write!(f, "{}", func),
            Value::Array(arr) =>
                Ok(array::display(arr.clone(), 1)),

            Value::Class(cls) =>
                write!(f, "{}", cls),
            Value::Object(obj) =>
                Ok(object::display(obj.clone(), 1))
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

    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Void(_)   => ValueType::Void,

            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
            Value::Array(_)  => ValueType::Array,
            Value::LazyExpression(_) => ValueType::LazyExpression,

            Value::Function(_) => ValueType::Function,
            Value::Class(_)    => ValueType::Class,
            Value::Object(_)   => ValueType::Object,
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