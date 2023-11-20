use crate::public::run_time::scope::Scope;
use crate::public::value::Value;
use crate::utils::editor::tokenizer::{TokenType, TokenVec};

fn get_end_part(tokens: &TokenVec) -> Option<Vec<String>> {
    // example:
    // input1 : ["var", "=", "obj", ".", "prop"]
    // output1: ["prop", "obj"]
    // input2:  ["var2", "=", "var1"]
    // output2: ["var1"]

    // if last token is Token with content "."
    if let Some(token) = tokens.last() {
        if token.content.eq(".") {
            return None;
        }
    } else {
        // tokens is empty
        return None;
    }

    let mut result = vec![];
    let mut last_type = TokenType::Comment;

    // traverse the inputed TokenVec reversedly
    for t in tokens.iter().rev() {
        // avoid repeated typed token
        if t.type__ == last_type {
            break;
        }

        // only allow "." and identifier
        if t.type__ == TokenType::Identifier {
            result.push(t.content.clone());
        } else if t.type__ == TokenType::Symbol && t.content.eq(".") {
            // do nothing
        } else {
            break;
        }

        last_type = t.type__;
    }

    if result.is_empty() {
        return None;
    } else {
        return Some(result);
    }
}

pub fn analyze(tokens: &TokenVec, scope: &Scope) -> Option<Vec<String>> {
    let Some(mut end_part) = get_end_part(tokens) else {
        return None;
    };

    if end_part.len() == 1 {
        // variable complete
        let global_completer = scope.completer.as_ref().unwrap();
        let word_to_complete = &end_part[0];
        let candidates = global_completer.complete(word_to_complete);
        return Some(candidates);
    } else {
        // object property complete
        let root_object = {
            let obj_name = end_part.pop().unwrap();
            let value = scope.read_var(&obj_name);
            if let Ok(Value::Object(obj_value)) = value {
                obj_value.clone()
            } else {
                return None;
            }
        };

        let mut var_object = root_object;
        while end_part.len() > 1 {
            let prop_name = end_part.pop().unwrap();
            let sub_value = var_object.as_ref().borrow().get(&prop_name);
            if let Ok(Value::Object(sub_obj)) = sub_value {
                var_object = sub_obj
            } else {
                return None;
            }
        }

        let completer = var_object.as_ref().borrow().get_completer();
        let Some(completer) = completer else {
            unreachable!();
        };
        let candidates = completer.complete(&end_part[0]);
        return Some(candidates);
    }
}