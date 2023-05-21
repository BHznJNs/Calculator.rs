use crate::public::error::syntax_error;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::{Overload, Value};

pub fn operate(val1: Value, val2: Value, operator: Symbols) -> Result<Value, ()> {
    let result = if let (Value::Number(num1_ref), Value::Number(num2_ref)) = (&val1, &val2) {
        // operating value must be type of Number.
        let num1 = *num1_ref;
        let num2 = *num2_ref;
        match operator {
            Symbols::Plus => Value::Number(num1 + num2),
            Symbols::Minus => Value::Number(num1 - num2),
            Symbols::Multiply => Value::Number(num1 * num2),
            Symbols::Divide => Value::Number(num1 / num2),
            Symbols::Power => Value::Number(num1.pow(num2)),
            Symbols::LessThan => Value::Boolean(num1 < num2),
            Symbols::MoreThan => Value::Boolean(num1 > num2),
            Symbols::NotEqual => Value::Boolean(num1 != num2),
            Symbols::CompareEqual => Value::Boolean(num1 == num2),
            Symbols::LessThanEqual => Value::Boolean(num1 <= num2),
            Symbols::MoreThanEqual => Value::Boolean(num1 >= num2),
            _ => {
                println!("Unexpected symbol: '{}' at function 'operate'.", operator);
                return Err(());
            }
        }
    } else if let (Value::String(str1_ref), Value::String(str2_ref)) = (&val1, &val2) {
        let str1 = str1_ref.as_ref().borrow();
        let str2 = str2_ref.as_ref().borrow();
        match operator {
            Symbols::Plus => {
                let mut cloned = str1.clone();
                cloned.push_str(&str2);
                Value::create(cloned)
            }
            Symbols::CompareEqual => Value::Boolean(str1.eq(&*str2)),
            _ => return Err(syntax_error("invalid string operating")?),
        }
    } else if let (Value::Boolean(bool1), Value::Boolean(bool2)) = (&val1, &val2) {
        if operator == Symbols::CompareEqual {
            Value::Boolean(bool1 == bool2)
        } else {
            return Err(syntax_error("invalid boolean operating")?);
        }
    } else {
        return Err(syntax_error(
            "invalid computing expression because of computing token",
        )?);
    };
    Ok(result)
}
