use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::BuildInFnCall;

#[derive(PartialEq)]
pub enum FileSysFn {
    // TODO
}

impl BuildInFnCall for FileSysFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        todo!()
    }
}