pub mod basic;
pub mod math;
pub mod array;
pub mod string;
pub mod file_system;

use crate::public::value::value::Value;
use crate::public::run_time::scope::Scope;

pub trait BuildInFnCall {
    fn call(&self, scope: &mut Scope)
        -> Result<Value, ()>;
}