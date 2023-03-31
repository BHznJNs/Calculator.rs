use crate::public::compile_time::ast::ASTNode;

use super::tokenizer::tokenizer::tokenize;
use super::analyzer::analyzer::analyze;

pub fn compile(input: &String) -> Result<ASTNode, ()> {
    let tokens = tokenize(input)?;
    // // LOG
    // for t in &tokens {
    //     println!("{}", t);
    // }
    // println!("--- --- ---");

    let ast = analyze(tokens)?;

    Ok(ast)
}