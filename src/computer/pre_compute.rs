use std::collections::HashMap;

use crate::public::number::Number;
use crate::public::token::{TokenVec, Token, Overloaded};
use crate::public::symbols::Symbols;
use crate::public::types::Types;
use super::compute::compute;

// pre-process tokens and convert identifier-expression
// to Number values.
pub fn pre_compute(
    mut tokens: TokenVec,
    variables : &mut HashMap<String, Number>,
    build_in_funcs: &HashMap<&str, fn(f64) -> f64>,
) -> Result<TokenVec, ()> {
    let mut index = 0;
    let mut current_token_index = 0;

    while index < tokens.len() {
        let mut current = &tokens[index];

        if current.type__ == Types::Identifier {
            let identi = tokens
                .remove(index)
                .identi
                .unwrap();
            current_token_index += 1;

            // is Symbols::LeftParen || Symbols::Equal
            let next_token: Token = if index < tokens.len() {
                current_token_index += 1;
                tokens.remove(index)
            } else {
                // if current is the last token
                Token::create(
                    Types::Symbol,
                    Symbols::NotASymbol
                )
            };

            if next_token.type__ != Types::Paren && next_token.symbol != Symbols::LeftParen {
                // token as a variable
                if next_token.type__ != Types::Symbol {
                    println!("Invalid token: `{}` at token {}.", next_token, current_token_index);
                    return Err(())
                }
                // Next token must be Symbol
                if next_token.symbol == Symbols::Equal {
                    // assignment operation
                    let mut sub_tokens = TokenVec::new();
                    while index < tokens.len() {
                        sub_tokens.push(tokens.remove(index));
                        current_token_index += 1;
                    }
                    if sub_tokens.len() == 0 {
                        println!("Variable assignment error.");
                        return Err(())
                    }
                    let resolved_tokens = pre_compute(sub_tokens, variables, build_in_funcs)?;
                    let value = compute(resolved_tokens, variables, build_in_funcs)?;

                    variables.insert(identi, value);
                    continue;
                }
                // read variable value
                let optional_var = variables.get(&identi);

                match optional_var {
                    Some(val) => {
                        tokens.insert(index, next_token);
                        tokens.insert(index, Token::create(
                            Types::Number, *val
                        ));
                    },
                    None => {
                        println!("Unknown variable: `{}` at token {}.", identi, current_token_index);
                        return Err(())
                    },
                }
                continue;
            }

            // token as build-in function
            let optional_func = build_in_funcs.get(identi.as_str());
            if optional_func.is_none() {
                println!("Unknown function: `{}` at token {}.", identi, current_token_index);
                return Err(())
            }
            let func = optional_func.unwrap();

            current = &tokens[index];
            let mut sub_tokens = TokenVec::new();
            let mut paren_pair_count = 1;
            while index < tokens.len() {
                if index == (tokens.len() - 1) && current.symbol != Symbols::RightParen {
                    println!("Unmatched parentheses at token {}.", current_token_index);
                    return Err(())
                }

                if current.symbol == Symbols::LeftParen {paren_pair_count += 1}
                if current.symbol == Symbols::RightParen {paren_pair_count -= 1}
                if paren_pair_count == 0 {break}
    
                sub_tokens.push(tokens.remove(index));
                current = &tokens[index];
            }
            let resolved_tokens = pre_compute(sub_tokens, variables, build_in_funcs)?;
            let sub_result = compute(resolved_tokens, variables, build_in_funcs)?;
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
            current_token_index += 1;
        }
    }
    Ok(tokens)
}