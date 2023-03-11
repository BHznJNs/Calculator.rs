use crate::public::ast::ASTNode;

use super::{tokenizer, analyzer};

pub fn compile(input: String) -> Result<ASTNode, ()> {
    let tokens = tokenizer::tokenizer(input)?;
    let ast = analyzer::analyzer(tokens)?;

    Ok(ast)
}