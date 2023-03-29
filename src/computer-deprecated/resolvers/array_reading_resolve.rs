use crate::computer::expression_compute::expression_compute;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::{Value, ArrayLiteral};

fn index_resolve(
    expression_node: &ASTNode,
    global: &mut Global
) -> Result<usize, ()> {
    let index_value =
        expression_compute(expression_node, global)?;
    if let Value::Number(number_box) = index_value.as_ref() {
        let number_value = *number_box;
        if number_value < Number::Int(0) {
            println!("Index of an array should not be less than ZERO.");
            return Err(())
        }
        Ok(number_value.int_value() as usize)
    } else {
        println!("Invalid array index.");
        Err(())
    }
}

pub fn array_reading_resolve(
    node: &ASTNode,
    array: Option<ArrayLiteral>,
    is_top: bool,
    global: &mut Global,
) -> Result<Box<Value>, ()> {
    let current_array =
    if is_top {
        // get array name
        let arr_name =
        if let ASTNodeTypes::ArrayElementReading(arr_name_string) = &node.type__ {
            arr_name_string
        } else {
            println!("Analyzer error: array name missing.");
            return Err(())
        };
        match global.variables.get(arr_name) {
            Some(arr_box) => {
                if let Value::Array(actual_arr) = arr_box.as_ref() {
                    *actual_arr.to_owned()
                } else {
                    println!();
                    return Err(())
                }
            },
            _ => {
                println!("'{}' is not an array.", arr_name);
                return Err(())
            }
        }
    } else {
        array.unwrap()
    };

    let params =
        node.params
        .as_ref()
        .unwrap();
    let index_number =
        index_resolve(&params[0], global)?;
    let target_value = if index_number < current_array.len() {
        &current_array[index_number]
    } else {
        println!("Index out of range.");
        return Err(())
    };

    if params.len() > 1 {
        // read sub array
        Ok(array_reading_resolve(
            &params[1],
            Some(current_array),
            false,
            global,
        )?)
    } else {
        Ok(target_value.to_owned())
    }

}