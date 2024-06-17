use crate::utils::editor::tokenizer::{TokenType, TokenVec};

pub fn analyze(tokens: &TokenVec) -> Option<Vec<String>> {
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
