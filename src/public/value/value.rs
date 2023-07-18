use std::cell::{RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

use crossterm::style::Stylize;

use crate::public::env::ENV_OPTION;
use crate::public::error::{internal_error, InternalComponent};

use super::super::compile_time::ast::ast_enum::ASTNode;
use super::array::{ArrayLiteral, Array};
use super::function::{
    BuildInFunction, Function, UserDefinedFunction,
};
use super::number::Number;
use super::oop::class::Class;
use super::oop::object::Object;

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

pub const VALUE_TYPE_PAIRS: [(&'static str, ValueType); 15] = [
    ("_", ValueType::Void),
    ("Bool", ValueType::Boolean),
    ("Numb", ValueType::Number),
    ("Str", ValueType::String),
    ("Array", ValueType::Array),
    ("LazyExpr", ValueType::LazyExpression),
    ("Func", ValueType::Function),
    ("Obj", ValueType::Object),

    ("布尔", ValueType::Boolean),
    ("数字", ValueType::Number),
    ("字符串", ValueType::String),
    ("数组", ValueType::Array),
    ("懒表达式", ValueType::LazyExpression),
    ("函数", ValueType::Function),
    ("对象", ValueType::Object),
];

impl ValueType {
    pub fn is_valid_type(identi: &String) -> Option<ValueType> {
        for (type_name, type__) in VALUE_TYPE_PAIRS {
            if identi.eq(type_name) {
                return Some(type__);
            }
        }
        None
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Void => write!(f, "任意类型"),
            ValueType::Boolean => write!(f, "布尔值"),
            ValueType::Number => write!(f, "数字"),
            ValueType::String => write!(f, "字符串"),
            ValueType::Array => write!(f, "数组"),
            ValueType::LazyExpression => write!(f, "懒表达式"),
            ValueType::Function => write!(f, "函数"),
            ValueType::Class => write!(f, "类"),
            ValueType::Object => write!(f, "对象"),
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
    // Value::Void(..)
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

impl Value {
    // formater for string typed value
    pub fn str_format(&self) -> String {
        if unsafe { ENV_OPTION.support_ansi } {
            format!("\"{}\"", self.to_string().green())
        } else {
            format!("\"{}\"", self)
        }
    }

    pub fn get_i64(&self) -> Result<i64, ()> {
        // expected Number typed value to call this method
        let Value::Number(num) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Value::get_i64` invocation"
            )?)
        };
        return Ok(num.int_value());
    }
    pub fn get_f64(&self) -> Result<f64, ()> {
        // expected Number typed value to call this method
        let Value::Number(num) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Value::get_f64` invocation"
            )?)
        };
        return Ok(num.float_value());
    }
    pub fn get_bool(&self) -> bool {
        match self {
            Value::Boolean(bool_val) => *bool_val,
            Value::Number(num) => *num != Number::Int(0),
            Value::String(str) => str.as_ref().borrow().len() > 0,
            Value::Array(arr) => arr.as_ref().borrow().len() > 0,

            Value::Void(_) => false,
            Value::LazyExpression(_) | Value::Function(_) | Value::Class(_) | Value::Object(_) => {
                true
            }
        }
    }
    pub fn get_str(&self) -> Result<RefMut<String>, ()> {
        let Value::String(str) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Value::get_str` invocation"
            )?)
        };
        let temp = str.borrow_mut();
        return Ok(temp);
    }

    // since the `to_string` method returns the string to display,
    // it needs an extra method to get raw_string(string without ANSI code).
    pub fn to_raw_string(&self) -> String {
        match self {
            Value::Void(_) => self.to_string(),
            Value::Boolean(bool_val) => bool_val.to_string(),
            Value::Number(num) => num.to_string(),
            Value::String(str) => str.borrow().clone(),
            Value::Function(func) => func.to_string(),
            Value::Array(arr) => Array::join(&arr.borrow(), ", "),

            Value::LazyExpression(_) => String::from("【懒表达式】"),
            Value::Class(_) => String::from("【类】"),
            Value::Object(_) => String::from("【对象】"),
        }
    }

    pub fn unwrap(&self) -> Value {
        // Rc<Value> -> Value
        // Ref<Value> -> Value
        self.clone()
    }
    pub fn deep_clone(&self) -> Value {
        let result = match self {
            // Boolean and Number is primitive type,
            // can be directly cloned.
            Value::Boolean(_)
            | Value::Number(_)
            // Function and Class can not be modified,
            // can just clone their Rc.
            | Value::Function(_)
            | Value::Class(_) => self.clone(),

            Value::String(str) => {
                let cloned_str = str.as_ref().borrow().clone();
                Value::from(cloned_str)
            },

            // for `Array` and `Object` the two complex type,
            // recursive clone is needed.
            Value::Array(arr) =>
                Array::deep_clone(arr.clone()),
            Value::Object(obj) =>
                Object::deep_clone(obj.clone()),

            Value::LazyExpression(l_expr) => {
                let cloned_l_expr = l_expr.as_ref().clone();
                Value::from(cloned_l_expr)
            },

            // user-defined common variable can not be `void` typed,
            // so that it need not to implement
            // `deep_clone`.
            Value::Void(_) => unreachable!(),
        };
        return result;
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Void(_) => ValueType::Void,

            Value::Boolean(_) => ValueType::Boolean,
            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
            Value::Array(_) => ValueType::Array,
            Value::LazyExpression(_) => ValueType::LazyExpression,

            Value::Function(_) => ValueType::Function,
            Value::Class(_) => ValueType::Class,
            Value::Object(_) => ValueType::Object,
        }
    }
    pub fn check_type(&self, target_type: ValueType) -> bool {
        if target_type == ValueType::Void {
            // `void` type can be any type
            return true;
        }
        return self.get_type() == target_type;
    }
}

impl Into<Rc<RefCell<Value>>> for Value {
    fn into(self) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(self))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void(void_sign) => match void_sign {
                VoidSign::Continue => write!(f, "Void(Continue)"),
                VoidSign::Break(val) => write!(f, "Void({})", val),
                VoidSign::Empty => write!(f, "<Void>"),
            },

            Value::String(str) => write!(f, "{}", str.as_ref().borrow()),
            Value::Array(arr) => Array::display(f, arr, 1),
            Value::Class(cls) => write!(f, "{}", cls),
            Value::Object(obj) => Object::display(f, obj, 1),

            _ => {
                if unsafe { ENV_OPTION.support_ansi } {
                    match self {
                        Value::Boolean(bool_val) => {
                            write!(f, "{}", bool_val.to_string().dark_yellow())
                        }
                        Value::Number(num) => write!(f, "{}", num.to_string().yellow()),
                        Value::LazyExpression(_) => write!(f, "{}", "【懒表达式】".cyan()),
                        Value::Function(func) => write!(f, "{}", func.to_string().cyan()),
                        _ => unreachable!(),
                    }
                } else {
                    match self {
                        Value::Boolean(bool_val) => write!(f, "{}", bool_val),
                        Value::Number(num) => write!(f, "{}", num),
                        Value::LazyExpression(_) => write!(f, "{}", "【懒表达式】"),
                        Value::Function(func) => write!(f, "{}", func),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(Number::Int(value))
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(Number::Float(value))
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(Rc::new(RefCell::new(value)))
    }
}
impl From<ArrayLiteral> for Value {
    fn from(value: ArrayLiteral) -> Self {
        Value::Array(Rc::new(RefCell::new(value)))
    }
}
impl From<ASTNode> for Value {
    fn from(value: ASTNode) -> Self {
        Value::LazyExpression(Rc::new(value))
    }
}

impl From<UserDefinedFunction> for Value {
    fn from(value: UserDefinedFunction) -> Self {
        Value::Function(Function::from(value))
    }
}
impl From<BuildInFunction> for Value {
    fn from(value: BuildInFunction) -> Self {
        Value::Function(Function::from(value))
    }
}
impl From<Class> for Value {
    fn from(value: Class) -> Self {
        Value::Class(Rc::new(value))
    }
}
impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Value::Object(Rc::new(RefCell::new(value)))
    }
}
