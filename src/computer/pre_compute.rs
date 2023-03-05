use std::collections::HashMap;

use crate::public::number::Number;
use crate::public::token::{TokenVec, Token, Overloaded};
use crate::public::symbols::Symbols;
use crate::public::types::Types;
use super::compute::compute;

// pre-process tokens and convert identifier-expression
// to Number values.
pub fn pre_compute(mut tokens: TokenVec, build_in_funcs: &HashMap<&str, fn(f64) -> f64>) -> Result<TokenVec, ()> {
    let mut index = 0;
    let mut token_count = tokens.len();

    while index < token_count {
        let mut current = &tokens[index];

        if current.type__ == Types::Identifier {
            let identi = &tokens
                .remove(index)
                .identi
                .unwrap();
            let optional_func = build_in_funcs.get(identi.as_str());
            if optional_func == None {
                println!("Unknown function name: `{}`.", identi);
                return Err(())
            }
            let func = optional_func.unwrap();

            let next_token = &tokens[index];

            if next_token.type__ != Types::Paren && next_token.symbol != Symbols::LeftParen {
                println!("Unknown token: `{}` at index {}.", next_token, index);
                return Err(())
            }

            index += 1;
            current = &tokens[index];
            let mut sub_tokens = TokenVec::new();
            let mut paren_pair_count = 1;
            while index < token_count {
                if index == (token_count - 1) && current.symbol != Symbols::RightParen {
                    println!("Unmatched parentheses at index {}.", index - 1);
                    return Err(())
                }

                if current.symbol == Symbols::LeftParen {paren_pair_count += 1}
                if current.symbol == Symbols::RightParen {paren_pair_count -= 1}
                if paren_pair_count == 0 {break}
    
                sub_tokens.push(tokens.remove(index));
                token_count = tokens.len();
                current = &tokens[index];
            }
            let sub_result = compute(sub_tokens, build_in_funcs)?;
            let value: Number;

            match sub_result {
                Number::Int(i)   => {value = Number::Float(func(i as f64))},
                Number::Float(f) => {value = Number::Float(func(f))},
            }
            tokens.insert(
                index,
                Token::create(
                    Types::Number,
                    value,
            ));
        } else {
            index += 1;
        }
    }
    Ok(tokens)
}