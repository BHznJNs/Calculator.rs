use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::value::parens::Parens;

use super::array;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    // no `new` keyword
    // example:
    // Person["BHznJNs", 99] | from `new Person["BHznJNs", 99]`

    let option_target_class =
        tokens.pop_front();
    let Some(Token::Identi(target_class)) =
        option_target_class else {
        println!("Expexted class name, which is missing.");
        return Err(())
    };

    // expect: `[`
    if tokens.pop_front() != Some(Token::Paren(Parens::LeftBracket)) {
        println!("Expected params for object instantiation.");
        return Err(())
    }

    let instantiation_params =
        array::literal_resolve(tokens)?;

    Ok(ASTNode {
        type__: ASTNodeTypes::Instantiation(target_class),
        params: Some(vec![instantiation_params])
    })
}