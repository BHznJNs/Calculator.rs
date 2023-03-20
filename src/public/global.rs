use std::collections::HashMap;

use super::ast::ASTNode;
use super::build_in;
use super::value::number::Number;

pub struct Global {
    pub build_in_funcs: HashMap<&'static str, fn(f64) -> f64>,
    pub variables: HashMap<String, Number>,
    pub lazy_expressions: HashMap<String, ASTNode>,
}

impl Global {
    pub fn init() -> Global {
        let build_in_inst = build_in::BuildIn::init();

        let instance = Global {
            build_in_funcs: build_in::build_in_funcs(&build_in_inst),
            variables: build_in::variables(&build_in_inst),
            lazy_expressions: HashMap::<String, ASTNode>::new(),
        };
        return instance
    }
}