use crate::public::error::{internal_error, syntax_error, InternalComponent};
use crate::public::value::symbols::Symbols;
use crate::public::value::value::Value;

pub fn operate(val1: Value, val2: Value, operator: Symbols) -> Result<Value, ()> {
    let result = match (&val1, &val2, operator) {
        (Value::Number(num1_ref), Value::Number(num2_ref), _) => {
            // number computing and comparing
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
                Symbols::AndSign => Value::Boolean(num1.int_value() != 0 && num2.int_value() != 0),
                Symbols::OrSign => Value::Boolean(num1.int_value() != 0 || num2.int_value() != 0),
                _ => {
                    let msg = format!("unexpected symbol `{}` for operating", operator);
                    return Err(internal_error(InternalComponent::Computer, &msg)?);
                }
            }
        }
        (Value::String(str_ref), _, Symbols::Plus) => {
            // stringify computing
            let mut str_cloned = str_ref.borrow().clone();
            let val2_str = val2.to_raw_string();
            str_cloned.extend(val2_str.chars());
            Value::from(str_cloned)
        }
        (_, _, Symbols::NotEqual | Symbols::CompareEqual | Symbols::AndSign | Symbols::OrSign) =>
        // all typed value comparing
        {
            match operator {
                Symbols::NotEqual => Value::Boolean(val1 != val2),
                Symbols::CompareEqual => Value::Boolean(val1 == val2),
                Symbols::AndSign => Value::Boolean(val1.get_bool() && val2.get_bool()),
                Symbols::OrSign => Value::Boolean(val1.get_bool() || val2.get_bool()),
                _ => unreachable!(),
            }
        }
        _ => return Err(syntax_error("invalid computing expression")?),
    };
    return Ok(result);
}
