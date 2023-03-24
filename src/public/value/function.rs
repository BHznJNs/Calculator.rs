use crate::public::compile_time::ast::ASTNodeVec;

pub struct Param {
    // type__: 
    name: String,
}

pub struct BuildInFunction {
    params: Vec<Param>,
    // body: 
}

pub struct UserDefinedFunction {
    params: Vec<Param>,
    body: ASTNodeVec,
}