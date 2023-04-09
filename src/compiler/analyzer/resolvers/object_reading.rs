use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::compiler::tokenizer::token::{Token, TokenVec};

pub fn resolve(
    obj_name: &String,
    tokens: &mut TokenVec
) -> Result<ASTNode, ()> {
    // object property / method reading

    let Some(Token::Identi(prop_name)) = tokens.pop_front() else {
        println!("Object property reading error.");
        return Err(())
    };

    let mut object_reading_params = vec![
        ASTNode {
            type__: ASTNodeTypes::ObjectReading(prop_name),
            params: None,
        }
    ];
    
    // nested reading
    let first_index = 0;
    while first_index < tokens.len() {
        let next_token = tokens.pop_front().unwrap();
        if next_token == Token::Symbol(Symbols::ObjectReading) {
            let Some(Token::Identi(prop_name)) = tokens.pop_front() else {
                println!("Object property reading error.");
                return Err(())
            };
            object_reading_params.push(ASTNode {
                type__: ASTNodeTypes::ObjectReading(prop_name),
                params: None,
            })
        } else {
            tokens.push_front(next_token);
            break;
        }
    }

    Ok(ASTNode {
        type__: ASTNodeTypes::ObjectReading(obj_name.clone()),
        params: Some(object_reading_params),
    })
}