use crate::public::compile_time::ast::ASTNodeVec;
use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::std::std::StdModules;

use super::value::ValueTypes;

pub struct BuildInParam {
    pub type__: ValueTypes,
    pub identi: &'static str,
}
pub struct BuildInFunction {
    pub params: [Option<BuildInParam>; 3],
    pub lib: StdModules,
    pub body: BuildInFuncs,
}

#[derive(PartialEq, Clone)]
pub struct Param {
    pub type__: ValueTypes,
    pub identi: String,
}

#[derive(PartialEq)]
pub struct UserDefinedFunction {
    pub params: Vec<Param>,
    pub body: ASTNodeVec,
}