use crate::public::compile_time::ast::ast_enum::RootNode;

use super::analyzer::analyzer::analyze;
use super::tokenizer;

pub fn compile(input: &String) -> Result<RootNode, ()> {
    let tokens = tokenizer::tokenize(input)?;
    // // LOG
    // for t in &tokens {
    //     println!("{}", t);
    // }
    // println!("--- --- ---");

    let ast = analyze(tokens)?;

    Ok(ast)
}
