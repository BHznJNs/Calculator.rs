use crate::public::run_time::scope::Scope;
use crate::public::value::Value;
use crate::utils::editor::tokenizer::{TokenType, TokenVec};

fn get_end_token(tokens: &TokenVec) -> Option<Vec<String>> {
    // example:
    // input1 : ["var", "=", "obj", ".", "prop"]
    // output1: ["prop", "obj"]
    // input2:  ["var2", "=", "var1"]
    // output2: ["var1"]

    let mut result_tokens: Vec<String> = vec![];
    let mut cached_identi: Option<&str> = None;

    for token in tokens.iter().rev() {
        if token.type__ == TokenType::Comment {
            continue;
        }

        match token.type__ {
            TokenType::Identifier | TokenType::Keyword if cached_identi.is_none() =>
                cached_identi = Some(&token.content),
            TokenType::Symbol if cached_identi.is_some() && token.content.eq(".") => {
                let identi = cached_identi.unwrap().to_owned();
                result_tokens.push(identi);
                cached_identi = None;
            },
            _ => break
        }
    }

    if let Some(identi) = cached_identi {
        result_tokens.push(identi.to_owned());
    }

    if result_tokens.is_empty() {
        return None;
    }

    return Some(result_tokens);
}

pub fn analyze(tokens: &TokenVec, scope: &Scope) -> Option<Vec<String>> {
    let mut end_part = get_end_token(tokens)?;

    if end_part.len() == 1 {
        // variable complete
        let global_completer = scope.completer.as_ref().unwrap();
        let word_to_complete = &end_part[0];
        let candidates = global_completer.complete(word_to_complete);
        return Some(candidates);
    }

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

    let completer = var_object.as_ref().borrow().get_completer()?;
    let candidates = completer.complete(&end_part[0]);
    return Some(candidates);
}

#[test]
fn get_end_token_test() {
    use crate::utils::editor::tokenizer::Token;
    let input = vec![
        Token { type__: TokenType::Identifier, content: String::from("obj")   },
        Token { type__: TokenType::Symbol    , content: String::from(".")     },
        Token { type__: TokenType::Identifier, content: String::from("prop1") },
        Token { type__: TokenType::Symbol    , content: String::from(".")     },
        Token { type__: TokenType::Identifier, content: String::from("prop2") },
    ];
    let output = get_end_token(&input);
    assert_eq!(output, Some(vec![
        String::from("prop2"),
        String::from("prop1"),
        String::from("obj"),
    ]));
}
