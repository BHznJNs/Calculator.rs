use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::Value;

pub fn operate(
    val1: Value,
    val2: Value,
    operator: Symbols
)-> Result<Value, ()> {
    let result = 
    if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
        match operator {
            Symbols::Plus     => num1 + num2,
            Symbols::Minus    => num1 - num2,
            Symbols::Multiply => num1 * num2,
            Symbols::Divide   => num1 / num2,
            Symbols::Power    => num1.pow(num2),
            Symbols::LessThan => Number::Int((num1 < num2) as i64),
            Symbols::MoreThan => Number::Int((num1 > num2) as i64),
            Symbols::LessThanEqual => Number::Int((num1 <= num2) as i64),
            Symbols::MoreThanEqual => Number::Int((num1 >= num2) as i64),
            Symbols::CompareEqual  => Number::Int((num1 == num2) as i64),
            _ => {
                println!("Unexpected symbol: '{}' at function 'operate'.", operator);
                return Err(())
            },
        }
    } else {
        println!("Invalid computing expression.");
        return Err(())
    };
    Ok(Value::Number(result))
}
