use std::cell::{RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

use crossterm::style::Stylize;

use crate::public::env::ENV_OPTION;
use crate::public::error::{internal_error, InternalComponent};

use super::super::compile_time::ast::ast_enum::ASTNode;
use super::array::{ArrayLiteral, RawArray, self};
use super::function::{BuildInFunction, Function, UserDefinedFunction};
use super::number::Number;
use super::oop::class::Class;
use super::oop::object::{self, Object};
use super::{GetAddr, into_rc_refcell};

#[cfg_attr(debug_assertions, derive(Debug))]
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
    ("_", ValueType::Void),
    ("Bool", ValueType::Boolean),
    ("Numb", ValueType::Number),
    ("Str", ValueType::String),
    ("Array", ValueType::Array),
    ("LazyExpr", ValueType::LazyExpression),
    ("Func", ValueType::Function),
    ("Obj", ValueType::Object),
];

impl ValueType {
    pub fn is_valid_type(identi: &String) -> Option<ValueType> {
        for (type_name, type__) in VALUE_TYPE_PAIRS {
            if identi.eq(type_name) {
                return Some(type__);
            }
        }
        return None;
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Void => write!(f, "Void"),
            ValueType::Boolean => write!(f, "Boolean"),
            ValueType::Number => write!(f, "Number"),
            ValueType::String => write!(f, "String"),
            ValueType::Array => write!(f, "Array"),
            ValueType::LazyExpression => write!(f, "LazyExpression"),
            ValueType::Function => write!(f, "Function"),
            ValueType::Class => write!(f, "Class"),
            ValueType::Object => write!(f, "Object"),
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
#[derive(Clone)]
pub enum Value {
    // Value::Void(..)
    // is used when comment line
    // or blank line or
    // or return state for statement.
    Void(VoidSign),

    Boolean(bool),
    Number(Number),
    String(Rc<RefCell<String>>),
    Array(Rc<RefCell<RawArray>>),
    LazyExpression(Rc<RefCell<ASTNode>>),

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
        let Self::Number(num) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Value::get_i64` invocation"
            )?)
        };
        return Ok(num.int_value());
    }
    pub fn get_f64(&self) -> Result<f64, ()> {
        // expected Number typed value to call this method
        let Self::Number(num) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Value::get_f64` invocation"
            )?)
        };
        return Ok(num.float_value());
    }
    pub fn get_bool(&self) -> bool {
        match self {
            Self::Boolean(bool_val) => *bool_val,
            Self::Number(num) => *num != Number::Int(0),
            Self::String(str) => str.as_ref().borrow().len() > 0,
            Self::Array(arr) => arr.as_ref().borrow().len() > 0,

            Self::Void(_) => false,
            Self::LazyExpression(_) | Self::Function(_) | Self::Class(_) | Self::Object(_) => true,
        }
    }
    pub fn get_str(&self) -> Result<RefMut<String>, ()> {
        let Self::String(str) = self else {
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
            Self::Void(_) => self.to_string(),
            Self::Boolean(bool_val) => bool_val.to_string(),
            Self::Number(num) => num.to_string(),
            Self::String(str) => str.borrow().clone(),
            Self::Function(func) => func.to_string(),
            Self::Array(arr) => arr.borrow().join(", "),

            Self::LazyExpression(_) => String::from("<Lazy-Expression>"),
            Self::Class(_) => String::from("<Class>"),
            Self::Object(_) => String::from("<Object>"),
        }
    }

    pub fn unwrap(&self) -> Self {
        // Rc<Value> -> Value
        // Ref<Value> -> Value
        self.clone()
    }
    pub fn deep_clone(&self) -> Self {
        let result = match self {
            // Boolean and Number is primitive type,
            // can be directly cloned.
            Self::Boolean(_)
            | Self::Number(_)
            // Function and Class can not be modified,
            // can just clone their Rc.
            | Self::Function(_)
            | Self::Class(_) => self.clone(),

            Self::String(str) => {
                let cloned_str = str.as_ref().borrow().clone();
                Self::from(cloned_str)
            },

            // for `Array` and `Object` the two complex type,
            // recursive clone is needed.
            Self::Array(arr) =>
                array::deep_clone(arr),
            Self::Object(obj) =>
                object::deep_clone(obj.clone()),

            Self::LazyExpression(l_expr) => {
                let cloned_l_expr = l_expr.as_ref().borrow().clone();
                Self::from(cloned_l_expr)
            },

            // user-defined common variable can not be `void` typed,
            // so that it need not to implement
            // `deep_clone`.
            Self::Void(_) => unreachable!(),
        };
        return result;
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            Self::Void(_) => ValueType::Void,

            Self::Boolean(_) => ValueType::Boolean,
            Self::Number(_) => ValueType::Number,
            Self::String(_) => ValueType::String,
            Self::Array(_) => ValueType::Array,
            Self::LazyExpression(_) => ValueType::LazyExpression,

            Self::Function(_) => ValueType::Function,
            Self::Class(_) => ValueType::Class,
            Self::Object(_) => ValueType::Object,
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void(void_sign) => match void_sign {
                VoidSign::Continue => write!(f, "Void(Continue)"),
                VoidSign::Break(val) => write!(f, "Void({})", val),
                VoidSign::Empty => write!(f, "<Void>"),
            },

            Self::String(str) => write!(f, "{}", str.as_ref().borrow()),
            Self::Array(arr) => array::display(f, arr, 1),
            Self::Class(cls) => write!(f, "{}", cls),
            Self::Object(obj) => object::display(f, obj, 1),

            _ => {
                if unsafe { ENV_OPTION.support_ansi } {
                    match self {
                        Self::Boolean(bool_val) => {
                            write!(f, "{}", bool_val.to_string().dark_yellow())
                        }
                        Self::Number(num) => write!(f, "{}", num.to_string().yellow()),
                        Self::LazyExpression(_) => write!(f, "{}", "<Lazy-Expression>".cyan()),
                        Self::Function(func) => write!(f, "{}", func.to_string().cyan()),
                        _ => unreachable!(),
                    }
                } else {
                    match self {
                        Self::Boolean(bool_val) => write!(f, "{}", bool_val),
                        Self::Number(num) => write!(f, "{}", num),
                        Self::LazyExpression(_) => write!(f, "{}", "<Lazy-Expression>"),
                        Self::Function(func) => write!(f, "{}", func),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

impl GetAddr for Value {
    fn get_addr(&self) -> super::Addr {
        match self {
            Self::Array(arr) => arr.as_ptr() as super::Addr,
            Self::LazyExpression(lexpr) => (&*lexpr.borrow() as *const ASTNode) as usize,
            Self::Function(func) => func.get_addr(),
            Self::Class(cls) => cls.get_addr(),
            Self::Object(obj) => obj.borrow().get_addr(),
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if self.get_type() != other.get_type() {
            return false;
        }

        match (self, other) {
            (Self::Void(sub1), Self::Void(sub2)) => sub1 == sub2,
            (Self::Boolean(bool_val1), Self::Boolean(bool_val2)) => *bool_val1 == *bool_val2,
            (Self::Number(num1), Self::Number(num2)) => *num1 == *num2,
            (Self::String(str_ref1), Self::String(str_ref2)) => {
                let str1 = str_ref1.borrow();
                let temp = str_ref2.borrow();
                let str2 = temp.as_str();
                str1.eq(str2)
            }
            (Self::LazyExpression(_), Self::LazyExpression(_))
            | (Self::Array(_), Self::Array(_))
            | (Self::Function(_), Self::Function(_))
            | (Self::Class(_), Self::Class(_))
            | (Self::Object(_), Self::Object(_)) => self.get_addr() == other.get_addr(),
            _ => unreachable!(),
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Number(Number::Int(value))
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(Number::Float(value))
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(into_rc_refcell(value))
    }
}
impl From<ArrayLiteral> for Value {
    fn from(value: ArrayLiteral) -> Self {
        Self::Array(into_rc_refcell(RawArray::new(value)))
    }
}
impl From<ASTNode> for Value {
    fn from(value: ASTNode) -> Self {
        Self::LazyExpression(into_rc_refcell(value))
    }
}

impl From<UserDefinedFunction> for Value {
    fn from(value: UserDefinedFunction) -> Self {
        Self::Function(Function::from(value))
    }
}
impl From<BuildInFunction> for Value {
    fn from(value: BuildInFunction) -> Self {
        Self::Function(Function::from(value))
    }
}
impl From<Class> for Value {
    fn from(value: Class) -> Self {
        Self::Class(Rc::new(value))
    }
}
impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Object(into_rc_refcell(value))
    }
}
