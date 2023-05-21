pub mod array;
pub mod basic;
pub mod file_system;
pub mod math;
pub mod string;

use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub trait BuildInFnCall {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()>;
}
