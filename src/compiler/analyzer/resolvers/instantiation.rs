use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::types::InstantiationNode;
use crate::public::error::syntax_error;
use crate::public::value::parens::Parens;

use super::array;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<InstantiationNode, ()> {
    // no `new` keyword
    // example:
    // Person["test", 99] | from `new Person["test", 99]`

    let Some(Token::Identi(target_class)) =
        tokens.pop_front() else {
        return Err(syntax_error("missing class name")?)
    };

    // expect: `[`
    if tokens.pop_front() != Some(Token::Paren(Parens::LeftBracket)) {
        return Err(syntax_error("missing params for object instantiation, expected '['")?)
    }

    let instantiation_params =
        array::literal_resolve(tokens)?;

    Ok(InstantiationNode {
        class: target_class,
        params: instantiation_params,
    })
}