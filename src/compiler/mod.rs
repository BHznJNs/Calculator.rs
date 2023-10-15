mod analyzer;
mod tokenizer;

use crate::public::{compile_time::ast::ast_enum::RootNode, error::CalcResult};

use analyzer::analyze;
use tokenizer::tokenize;

pub fn compile(input: &str) -> CalcResult<RootNode> {
    let tokens = tokenize(input)?;
    let ast = analyze(tokens)?;
    return Ok(ast);
}
