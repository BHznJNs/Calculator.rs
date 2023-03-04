use super::compiler::tokenizer::Token;
use super::compiler::symbols::Symbols;
use super::compiler::types::Types;

pub fn compute(tokens: Vec<Token>) -> i32 {
    let mut number_stack = Vec::<i32>::new();
    let mut symbol_stack = Vec::<Symbols>::new();

    let mut index = 0;
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
                if current.symbol == Symbols::Multiply || current.symbol == Symbols::Divide {
                    index += 1;
                    let num2 = tokens[index].number;
                    let num1 = number_stack.pop().unwrap();

                    if current.symbol == Symbols::Multiply {
                        number_stack.push(num1 * num2);
                    }
                    if current.symbol == Symbols::Divide {
                        number_stack.push(num1 / num2);
                    }
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

                    number_stack.push(compute(sub_tokens));
                }
            },
        }
        index += 1;
    }
    for symbol in symbol_stack {
        let num2 = number_stack.pop().unwrap();
        let num1 = number_stack.pop().unwrap();

        if symbol == Symbols::Plus {
            number_stack.push(num1 + num2);
        }
        if symbol == Symbols::Minus {
            number_stack.push(num1 - num2);
        }
    }
    return number_stack[0];
}