use crate::public::compile_time::ast::ast_enum::ASTNode;
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
                Symbols::LessThan => Value::from(num1 < num2),
                Symbols::MoreThan => Value::from(num1 > num2),
                Symbols::NotEqual => Value::from(num1 != num2),
                Symbols::CompareEqual => Value::from(num1 == num2),
                Symbols::LessThanEqual => Value::from(num1 <= num2),
                Symbols::MoreThanEqual => Value::from(num1 >= num2),
                Symbols::AndSign => Value::from(num1.int_value() != 0 && num2.int_value() != 0),
                Symbols::OrSign => Value::from(num1.int_value() != 0 || num2.int_value() != 0),
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
        (
            Value::LazyExpression(lazy_expr),
            Value::Number(_) | Value::String(_) | Value::LazyExpression(_),
            _,
        ) => {
            // lazy expression computing

            // `lazy_expr` sample structure:
            // Expression(
            //     ExpressionNode {
            //         elements: [
            //             NumberLiteral(
            //                 Int(
            //                     1,
            //                 ),
            //             ),
            //             NumberLiteral(
            //                 Int(
            //                     1,
            //                 ),
            //             ),
            //             SymbolLiteral(
            //                 Plus,
            //             ),
            //         ],
            //     },
            // )
            let mut new_lazy_expr = lazy_expr.borrow().clone();
            let ASTNode::Expression(expr_node) = &mut new_lazy_expr else {
                unreachable!()
            };
            // push added value and Symbols
            let expr_elements = &mut expr_node.elements;

            match val2 {
                Value::Number(num) => {
                    expr_elements.push(ASTNode::NumberLiteral(num));
                }
                Value::String(str) => {
                    let cloned = str.borrow().clone();
                    expr_elements.push(ASTNode::StringLiteral(cloned));
                }
                Value::LazyExpression(other_lexpr) => {
                    let ASTNode::Expression(expr_node) = &*other_lexpr.borrow() else {
                        unreachable!()
                    };
                    let cloned_elements = expr_node.elements.clone();
                    expr_elements.extend(cloned_elements.into_iter());
                }
                _ => unreachable!(),
            }
            // expr_elements.push(ASTNode::NumberLiteral(*num));
            expr_elements.push(ASTNode::SymbolLiteral(operator));

            Value::from(new_lazy_expr)
        }
        (_, _, Symbols::NotEqual | Symbols::CompareEqual | Symbols::AndSign | Symbols::OrSign) =>
        // all typed value comparing
        {
            match operator {
                Symbols::NotEqual => Value::from(val1 != val2),
                Symbols::CompareEqual => Value::from(val1 == val2),
                Symbols::AndSign => Value::from(val1.get_bool() && val2.get_bool()),
                Symbols::OrSign => Value::from(val1.get_bool() || val2.get_bool()),
                _ => unreachable!(),
            }
        }
        _ => return Err(syntax_error("invalid computing expression")?),
    };
    return Ok(result);
}
