use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::tokenizer::{TextType, TokenVec};

fn get_end_part(tokens: &TokenVec) -> Option<Vec<String>> {
    // example:
    // input1 : ["var", "=", "obj", ".", "prop"]
    // output1: ["prop", "obj"]
    // input2:  ["var2", "=", "var1"]
    // output2: ["var1"]

    if tokens.is_empty() {
        return None;
    }

    let mut result = vec![];
    let mut index = tokens.len() - 1;
    let mut last_type = TextType::Comment;

    // traverse the inputed TokenVec reversedly
    loop {
        let current = &tokens[index];

        // avoid repeated typed token
        if current.type__ == last_type {
            break;
        }
        // only allow "." and identifier
        if current.type__ == TextType::Variable {
            result.push(current.content.clone());
        } else if current.type__ == TextType::Symbol && current.content.eq(".") {
            // do nothing
        } else {
            break;
        }

        if index == 0 {
            break;
        }

        last_type = current.type__;
        index -= 1;
    }

    if result.is_empty() {
        return None;
    } else {
        return Some(result);
    }
}

pub fn analyze(tokens: &TokenVec, scope: &Scope) -> Result<Vec<String>, ()> {
    let Some(mut end_part) = get_end_part(tokens) else {
        return Ok(vec![])
    };

    if end_part.len() == 1 {
        // variable complete
        let global_completer = scope.completer.as_ref().unwrap();
        let word_to_complete = &end_part[0];
        let candidates = global_completer.complete(word_to_complete);
        return Ok(candidates);
    } else {
        // object property complete
        let root_object = {
            let obj_name = end_part.pop().unwrap();
            let value = scope.read_var(&obj_name)?;
            if let Value::Object(obj_value) = value {
                obj_value.clone()
            } else {
                return Err(());
            }
        };

        let mut var_object = root_object;
        while end_part.len() > 1 {
            let prop_name = end_part.pop().unwrap();
            let sub_val = var_object.as_ref().borrow().get(&prop_name)?;
            if let Value::Object(sub_obj) = sub_val {
                var_object = sub_obj
            } else {
                return Err(());
            }
        }

        let target_proto = &var_object.as_ref().borrow().prototype;
        let Some(completer) = &target_proto.completer else {
            return Err(());
        };
        let candidates = completer.complete(&end_part[0]);
        return Ok(candidates);
    }
}
