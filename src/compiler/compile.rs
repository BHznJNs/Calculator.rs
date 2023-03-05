use super::tokenizer::tokenizer;
use crate::public::token::TokenVec;

pub fn compile(source: String) -> Result<TokenVec, ()> {
    let tokens = tokenizer(source)?;
    // for item in &tokens {
    //     // Check tokens
    //     println!("type: {} | value: {}", item.type__, item);
    // }
    Ok(tokens)
}