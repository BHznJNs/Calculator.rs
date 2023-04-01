use std::rc::Rc;

use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::Value;

pub fn operate(
    box1: Rc<Value>,
    box2: Rc<Value>,
    operator: Symbols
)-> Result<Value, ()> {
    let val1 = box1.as_ref();
    let val2 = box2.as_ref();

    let result =
    if let (Value::Number(n1), Value::Number(n2)) = (val1, val2) {
        // operating value must be type of Number.
        let num1 = *n1;
        let num2 = *n2;

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
    } else {
        println!("Invalid computing expression: Invalid computing token.");
        return Err(())
    };
    Ok(Value::Number(result))
}