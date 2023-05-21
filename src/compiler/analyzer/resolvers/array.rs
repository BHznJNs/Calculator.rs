use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::{ArrayElementReadingNode, ArrayLiteralNode};
use crate::public::compile_time::parens::Paren;

use super::{expression, list};

pub fn literal_resolve(tokens: &mut TokenVec) -> Result<ArrayLiteralNode, ()> {
    let elements = list::resolve(tokens, Paren::RightBracket)?;
    Ok(ArrayLiteralNode { elements })
}

// #[derive(PartialEq)]
// enum State {
//     Inner,
//     Outer,
// }

// // refactor: params_resolve
// pub fn literal_resolve(
//     tokens: &mut TokenVec
// ) -> Result<ArrayLiteralNode, ()> {
//     // example:
//     // 1, 2, 3, 4]
//     // 1, [2, 3], 4]

//     fn element_resolve(
//         sub_tokens: &mut TokenVec,
//         elements: &mut Vec<ExpressionNode>,
//     ) -> Result<(), ()> {
//         if sub_tokens.len() > 0 {
//             let element =
//                 expression::resolve(sub_tokens)?;
//             sub_tokens.clear();
//             elements.push(element);
//         }
//         Ok(())
//     }

//     let first_index = 0;
//     let mut state = State::Outer;
//     let mut paren_count = 1;
//     let mut elements = Vec::<ExpressionNode>::new();
//     let mut sub_tokens = TokenVec::new();

//     while first_index < tokens.len() {
//         let current =
//             tokens.pop_front().unwrap();

//         let is_divider =
//             current == Token::Divider;
//         let is_left_paren =
//             current == Token::Paren(Paren::LeftBrace) ||
//             current == Token::Paren(Paren::LeftParen) ||
//             current == Token::Paren(Paren::LeftBracket);
//         let is_right_paren =
//             current == Token::Paren(Paren::RightBrace) ||
//             current == Token::Paren(Paren::RightParen) ||
//             current == Token::Paren(Paren::RightBracket);

//         if is_left_paren {
//             state = State::Inner;
//             paren_count += 1;
//         }
//         if is_divider && (state == State::Outer) {
//             element_resolve(&mut sub_tokens, &mut elements)?;
//             continue;
//         }
//         if is_right_paren {
//             paren_count -= 1;
//             if paren_count == 1 {
//                 state = State::Outer;
//             }
//             if paren_count == 0 {
//                 element_resolve(&mut sub_tokens, &mut elements)?;
//                 break;
//             }
//         }

//         sub_tokens.push_back(current);
//     }

//     Ok(ArrayLiteralNode { elements })
// }

pub fn reading_resolve(
    array_node: ASTNode,
    tokens: &mut TokenVec,
) -> Result<ArrayElementReadingNode, ()> {
    // example:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`

    let first_index = 0;
    let mut bracket_count = 1;
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Paren::LeftBracket) {
            bracket_count += 1;
        }
        if current == Token::Paren(Paren::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }
    let index_node = expression::resolve(&mut sub_tokens)?;
    Ok(ArrayElementReadingNode {
        array_node,
        index_node,
    })
}
