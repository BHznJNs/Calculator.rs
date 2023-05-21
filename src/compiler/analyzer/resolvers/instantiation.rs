use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::{ArrayLiteralNode, InstantiationNode};
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;

use super::list;

pub fn resolve(tokens: &mut TokenVec) -> Result<InstantiationNode, ()> {
    // no `new` keyword
    // example:
    // Person["test", 99] | from `new Person["test", 99]`

    let Some(Token::Identi(target_class)) =
        tokens.pop_front() else {
        return Err(syntax_error("missing class name")?)
    };

    // expect: `[`
    if tokens.pop_front() != Some(Token::Paren(Paren::LeftParen)) {
        return Err(syntax_error(
            "missing params for object instantiation, expected '['",
        )?);
    }

    let instantiation_params = list::resolve(tokens, Paren::RightParen)?;
    // let instantiation_params =
    //     array::literal_resolve(tokens)?;

    Ok(InstantiationNode {
        class: target_class,
        params: ArrayLiteralNode {
            elements: instantiation_params,
        },
    })
}
