use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::compiler::tokenizer::token::{Token, TokenVec};

pub fn resolve(
    obj_node: Rc<ASTNode>,
    tokens: &mut TokenVec
) -> Result<ASTNode, ()> {
    // object property / method reading

    let Some(Token::Identi(prop_name)) = tokens.pop_front() else {
        println!("Object property reading error.");
        return Err(())
    };
    let object_reading_param = ASTNode {
        type__: ASTNodeTypes::Variable(prop_name),
        params: None,
    };

    Ok(ASTNode {
        type__: ASTNodeTypes::ObjectReading(obj_node),
        params: Some(vec![object_reading_param]),
    })
}