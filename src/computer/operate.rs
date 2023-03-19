use crate::public::number::Number;
use crate::public::symbols::Symbols;

pub fn operate(num1: Number, num2: Number, operator: Symbols) -> Result<Number, ()> {
    match operator {
        Symbols::Plus     => Ok(num1 + num2),
        Symbols::Minus    => Ok(num1 - num2),
        Symbols::Multiply => Ok(num1 * num2),
        Symbols::Divide   => Ok(num1 / num2),
        Symbols::Power    => Ok(num1.pow(num2)),
        Symbols::LessThan => Ok(Number::Int((num1 < num2) as i64)),
        Symbols::MoreThan => Ok(Number::Int((num1 > num2) as i64)),
        Symbols::LessThanEqual => Ok(Number::Int((num1 <= num2) as i64)),
        Symbols::MoreThanEqual => Ok(Number::Int((num1 >= num2) as i64)),
        Symbols::CompareEqual  => Ok(Number::Int((num1 == num2) as i64)),
        _ => {
            println!("Unexpected symbol: '{}' at function `operate`.", operator);
            Err(())
        },
    }
}
