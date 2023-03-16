use crate::public::number::Number;
use crate::public::symbols::Symbols;

pub fn operate(num1: Number, num2: Number, operator: Symbols) -> Result<Number, ()> {
    match operator {
        Symbols::Plus     => {Ok(num1 + num2)},
        Symbols::Minus    => {Ok(num1 - num2)},
        Symbols::Multiply => {Ok(num1 * num2)},
        Symbols::Divide   => {Ok(num1 / num2)},
        Symbols::Power    => {Ok(num1.pow(num2))},
        _                 => {
            println!("Unexpected symbol: '{}' at function `operate`.", operator);
            Err(())
        },
    }
}
