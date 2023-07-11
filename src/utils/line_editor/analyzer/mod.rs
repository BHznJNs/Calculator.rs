use crate::public::run_time::scope::Scope;

use super::tokenizer::{TokenVec, TextType};

fn get_end_part(tokens: &TokenVec) -> Option<Vec<String>> {
    if tokens.is_empty() {
        return None;
    }

    let mut result = vec![];
    let mut index = tokens.len() - 1;
    let mut last_type = TextType::Comment;

    loop {
        let current = &tokens[index];

        if current.type__ == last_type {
            break;
        }

        if current.type__ == TextType::Variable 
            || (current.type__ == TextType::Symbol && current.content.eq(".")) {
            result.push(current.content.clone());
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

pub fn analyze(tokens: &TokenVec, scope: &Scope) -> Vec<String> {
    let Some(end_part) = get_end_part(tokens) else {
        return vec![]
    };

    if end_part.len() == 1 {
        // variable complete
        let global_completer = scope.completer.as_ref().unwrap();
        let word_to_complete = &end_part[end_part.len() - 1];
        return global_completer.complete(word_to_complete);
    } else {
        // object property complete
        todo!()
    }
}