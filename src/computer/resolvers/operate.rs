use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::{Value, Overload};

pub fn operate(
    val1: Value,
    val2: Value,
    operator: Symbols
)-> Result<Value, ()> {
    let result =
    if let (Value::Number(num1_ref), Value::Number(num2_ref)) = (&val1, &val2) {
        // operating value must be type of Number.
        let num1 = *num1_ref;
        let num2 = *num2_ref;
        match operator {
            Symbols::Plus     => num1 + num2,
            Symbols::Minus    => num1 - num2,
            Symbols::Multiply => num1 * num2,
            Symbols::Divide   => num1 / num2,
            Symbols::Power    => num1.pow(num2),
            Symbols::LessThan => Number::Int((num1 < num2) as i64),
            Symbols::MoreThan => Number::Int((num1 > num2) as i64),
            Symbols::CompareEqual  => Number::Int((num1 == num2) as i64),
            Symbols::LessThanEqual => Number::Int((num1 <= num2) as i64),
            Symbols::MoreThanEqual => Number::Int((num1 >= num2) as i64),
            _ => {
                println!("Unexpected symbol: '{}' at function 'operate'.", operator);
                return Err(())
            },
        }
    } else
    if let (Value::String(str1_ref), Value::String(str2_ref)) = (&val1, &val2) {
        let str1 = str1_ref.as_ref().borrow_mut();
        let str2 = str2_ref.as_ref().borrow();
        match operator {
            Symbols::Plus => {
                let mut cloned = str1.clone();
                cloned.push_str(&str2);
                return Ok(Value::create(cloned))
            },
            Symbols::CompareEqual =>
                Number::Int(str1.eq(&*str2) as i64),
            _ => {
                println!("Invalid string operate.");
                return Err(())
            }
        }
    } else {
        println!("Invalid computing expression: Invalid computing token.");
        return Err(())
    };
    Ok(Value::Number(result))
}