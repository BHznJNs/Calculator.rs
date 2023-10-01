pub mod compile_time;
pub mod run_time;

pub mod std;

pub mod env;
pub mod error;
pub mod value;

use value::ValueType;

pub trait Param {
    fn type__(&self) -> ValueType;
    fn identi(&self) -> &str;
}
