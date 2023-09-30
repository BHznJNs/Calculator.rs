mod analyzer;
mod tokenizer;

use crate::public::compile_time::ast::ast_enum::RootNode;

use analyzer::analyze;
use tokenizer::tokenize;

pub fn compile(input: &String) -> Result<RootNode, ()> {
    let tokens = tokenize(input)?;
    let ast = analyze(tokens)?;
    return Ok(ast);
}
