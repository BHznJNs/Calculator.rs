use crate::public::ast::ASTNode;

use super::tokenizer::tokenizer;
use super::analyzer::analyzer;

pub fn compile(input: String) -> Result<ASTNode, ()> {
    let tokens = tokenizer(input)?;
    // LOG
    // for t in &tokens {
    //     println!("{}", t);
    // }
    // println!("--- --- ---");
    let ast = analyzer(tokens)?;

    Ok(ast)
}