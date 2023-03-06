use std::collections::HashMap;

use crate::public::symbols::Symbols;
use crate::public::number::Number;
use crate::public::token::{Token, TokenVec, Overloaded};
use crate::public::types::Types;
use super::pre_compute::pre_compute;

pub fn operate(num1: Number, num2: Number, operator: Symbols) -> Number {
    match operator {
        Symbols::Plus     => {num1 + num2},
        Symbols::Minus    => {num1 - num2},
        Symbols::Multiply => {num1 * num2},
        Symbols::Divide   => {num1 / num2},
        Symbols::Power    => {num1.pow(num2)},
        _                 => {panic!("Unknown symbol: `{}` at function `operate`.", operator)},
    }
}

pub fn compute(
    mut tokens: TokenVec,
    variables : &mut HashMap<String, Number>,
    build_in_funcs: &HashMap<&str, fn(f64) -> f64>,
) -> Result<Number, ()> {
    let mut number_stack = Vec::<Number>::new();
    let mut symbol_stack = Vec::<Symbols>::new();

    let mut index = 0;
    let mut waiting_num: Option<Number> = None;

    if tokens.len() == 0 {return Err(())}

    while index < tokens.len() {
        let mut current = &tokens[index];

        match current.type__ {
            Types::Unknown => {},
            Types::Number => {
                number_stack.push(current.number);
            },
            Types::Identifier => {
                let mut sub_tokens = TokenVec::new();
                let mut paren_pair_count = 0;
                sub_tokens.push(tokens.remove(index));

                current = &tokens[index]; // next
                if current.symbol != Symbols::LeftParen {
                    println!("Unknown token: `{}` at index {}.", current, index);
                    return Err(())
                }
                while index < tokens.len() {
                    if index == (tokens.len() - 1) && current.symbol != Symbols::RightParen  {
                        println!("Unmatched parentheses at index {}.", index - 1);
                        return Err(())
                    }

                    if current.symbol == Symbols::LeftParen  {paren_pair_count += 1}
                    if current.symbol == Symbols::RightParen {paren_pair_count -= 1}
                    if paren_pair_count == 0 {break}

                    sub_tokens.push(tokens.remove(index));
                    current = &tokens[index];
                }
                sub_tokens.push(tokens.remove(index));

                let sub_tokens_resolved = pre_compute(sub_tokens, variables, build_in_funcs)?;
                let sub_token_value = compute(sub_tokens_resolved, variables, build_in_funcs)?;
                number_stack.push(sub_token_value);
                tokens.insert(index, Token::create(
                    Types::Number,
                    sub_token_value,
                ));
            },
            Types::Symbol => {
                if current.symbol == Symbols::Plus || current.symbol == Symbols::Minus {
                    symbol_stack.push(current.symbol);
                }
                // Multiply | Divide | Power
                if current.symbol == Symbols::Multiply || current.symbol == Symbols::Divide || current.symbol == Symbols::Power {
                    index += 1;
                    let num1 = number_stack.pop().unwrap();
                    let next_token = &tokens[index];
                    match next_token.type__ {
                        Types::Paren  => {
                            if next_token.symbol == Symbols::LeftParen {
                                waiting_num = Some(num1);
                                symbol_stack.push(current.symbol);
                                continue;
                            } else {
                                println!("Unknown symbol: `)` at index {}.", index);
                                return Err(())
                            }
                        },
                        Types::Symbol => {
                            println!("Unknown symbol: `{}` at index {}.", next_token.symbol, index);
                            return Err(())
                        },
                        _ => {}
                    }
                    let num2 = tokens[index].number;
                    number_stack.push(
                        operate(num1, num2, current.symbol)
                    )
                }
            },
            Types::Paren => {
                if current.symbol == Symbols::LeftParen {
                    let mut paren_pair_count = 1;

                    index += 1;
                    current = &tokens[index];
                    let mut sub_tokens = Vec::<Token>::new();

                    while index < tokens.len() {
                        if index == (tokens.len() - 1) && current.symbol != Symbols::RightParen {
                            println!("Unmatched parentheses at index {}.", index - 1);
                            return Err(())
                        }
                        if current.symbol == Symbols::LeftParen  {paren_pair_count += 1}
                        if current.symbol == Symbols::RightParen {paren_pair_count -= 1}
                        if paren_pair_count == 0 {break}

                        sub_tokens.push(tokens.remove(index));
                        current = &tokens[index];
                    }

                    let sub_result = compute(sub_tokens, variables, build_in_funcs)?;

                    match waiting_num {
                        Some(num1) => {
                            waiting_num = None;
                            let num2 = sub_result;
                            let operator = symbol_stack.pop().unwrap();
                            number_stack.push(operate(num1, num2, operator))
                        },
                        None => {number_stack.push(sub_result);}
                    }
                }
            },
        }
        index += 1;
    }
    // clear remain elements in number_stack and symbol_stack
    while !symbol_stack.is_empty() {
        if symbol_stack.len() < 1 {
            println!("Computing symbol missing.");
            return Err(())
        }
        if number_stack.len() < 2 {
            println!("Computing number missing.");
            return Err(())
        }

        let symbol = symbol_stack.remove(0);
        let num1 = number_stack.remove(0);
        let num2 = number_stack.remove(0);

        number_stack.insert(0, operate(
            num1, num2, symbol,
        ));
    }
    return Ok(number_stack[0])
}