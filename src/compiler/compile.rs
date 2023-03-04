use super::tokenizer::tokenizer;
use super::tokenizer::Token;

pub fn compile(source: String) -> Vec<Token> {
    let tokens = tokenizer(source);
    for item in &tokens {
        // Check tokens 
        println!("type: {} | number: {} | symbol: {}", item.type__, item.number, item.symbol);
    }
    return tokens;
}