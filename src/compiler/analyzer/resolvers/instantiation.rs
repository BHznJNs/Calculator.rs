use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::{ArrayLiteralNode, InstantiationNode};
use crate::public::compile_time::parens::Paren;
use crate::public::error::{syntax_error, CalcResult};

use super::list;

pub fn resolve(tokens: &mut TokenVec) -> CalcResult<InstantiationNode> {
    // no `new` keyword
    // example:
    // Person["test", 99] | from `new Person["test", 99]`

    let Some(Token::Identi(target_class)) = tokens.pop_front() else {
        return Err(syntax_error("missing class name"));
    };

    // expect: `[`
    if tokens.pop_front() != Some(Token::Paren(Paren::LeftParen)) {
        return Err(syntax_error(
            "missing params for object instantiation, expected '['",
        ));
    }
    let instantiation_params = list::resolve(tokens, Paren::RightParen)?;
    return Ok(InstantiationNode {
        class: target_class,
        params: ArrayLiteralNode {
            elements: instantiation_params,
        },
    });
}
