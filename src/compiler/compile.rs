use crate::public::compile_time::ast::ast_enum::RootNode;

use super::analyzer::analyzer::analyze;
use super::tokenizer::tokenizer::tokenize;

pub fn compile(input: &String) -> Result<RootNode, ()> {
    let tokens = tokenize(input)?;
    // // LOG
    // for t in &tokens {
    //     println!("{}", t);
    // }
    // println!("--- --- ---");

    let ast = analyze(tokens)?;

    Ok(ast)
}
