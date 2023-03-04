use super::compiler::tokenizer::Token;
use super::compiler::symbols::Symbols;
use super::compiler::types::Types;

fn operate(num1: i32, num2: i32, operator: Symbols) -> i32 {
    match operator {
        Symbols::Plus     => {num1 + num2},
        Symbols::Minus    => {num1 - num2},
        Symbols::Multiply => {num1 * num2},
        Symbols::Divide   => {num1 / num2},
        Symbols::Power    => {num1.pow(num2 as u32)},
        _                 => {panic!("Unknown symbol: `{}` at function `operate`.", operator)},
    }
}

pub fn compute(tokens: Vec<Token>) -> Result<i32, ()> {
    let mut number_stack = Vec::<i32>::new();
    let mut symbol_stack = Vec::<Symbols>::new();

    let mut index = 0;
    let mut waiting_num = 0;
    let token_count = tokens.len();

    while index < token_count {
        let mut current = &tokens[index];

        match current.type__ {
            Types::Number => {
                number_stack.push(current.number);
            },
            Types::Symbol => {
                if current.symbol == Symbols::Plus || current.symbol == Symbols::Minus {
                    symbol_stack.push(current.symbol);
                }
                // Multiply | Divide | Power
                if current.symbol == Symbols::Multiply || current.symbol == Symbols::Divide || current.symbol == Symbols::Power {
                    index += 1;
                    let num1 = number_stack.pop().unwrap();
                    let next_token = tokens[index];
                    match next_token.type__ {
                        Types::Number => {},
                        Types::Paren  => {
                            if next_token.symbol == Symbols::LeftParen {
                                waiting_num = num1;
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
                        }
                    }
                    let num2 = tokens[index].number;
                    number_stack.push(
                        operate(num1, num2, current.symbol)
                    )
                }
            },
            Types::Paren => {
                if current.symbol == Symbols::LeftParen {
                    index += 1;
                    current = &tokens[index];
                    let mut sub_tokens = Vec::<Token>::new();

                    while current.type__ != Types::Paren && current.symbol != Symbols::RightParen {
                        sub_tokens.push(current.clone());
                        index += 1;
                        current = &tokens[index];
                    }

                    let sub_result = compute(sub_tokens)?;

                    if waiting_num != 0 {
                        let num1 = waiting_num;
                        let num2 = sub_result;
                        waiting_num = 0;
                        let operator = symbol_stack.pop().unwrap();
                        number_stack.push(operate(num1, num2, operator))
                    } else {
                        number_stack.push(sub_result);
                    }
                }
            },
        }
        index += 1;
    }
    for symbol in symbol_stack {
        let num2 = number_stack.pop().unwrap();
        let num1 = number_stack.pop().unwrap();

        number_stack.push(
            operate(num1, num2, symbol)
        );
    }
    return Ok(number_stack[0])
}