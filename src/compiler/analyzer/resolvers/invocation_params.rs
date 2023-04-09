use crate::compiler::analyzer::resolvers::expression;
use crate::public::value::parens::Parens;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

pub fn resolve(
    tokens: &mut TokenVec
) -> Result<ASTNodeVec, ()> {
    // examples:
    // 1, 2)
    // a, 1)
    fn param_expr_resolve(
        sub_tokens: &mut TokenVec,
        params: &mut ASTNodeVec,
    ) -> Result<(), ()> {
        if sub_tokens.len() > 0 {
            let sub_expression =
                expression::resolve(sub_tokens, false)?;
            let sub_expression_node = ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(sub_expression),
            };
            params.push(sub_expression_node);
            sub_tokens.clear();
        }
        Ok(())
    }

    let first_index = 0;
    let mut paren_count = 1;
    let mut sub_tokens = TokenVec::new();
    let mut params = ASTNodeVec::new();

    loop {
        if first_index == tokens.len() {
            println!("Unmatched parentheses.");
            return Err(())
        }

        let current = tokens.pop_front().unwrap();

        let is_divider =
            current == Token::Divider;
        let is_left_paren =
            current == Token::Paren(Parens::LeftParen);
        let is_right_paren =
            current == Token::Paren(Parens::RightParen);

        if is_left_paren {
            paren_count += 1;
        }
        if is_divider {
            param_expr_resolve(&mut sub_tokens, &mut params)?;
        }
        if is_right_paren {
            paren_count -= 1;
            if paren_count == 0 {
                param_expr_resolve(&mut sub_tokens, &mut params)?;
                break;
            }
        }

        if !is_divider {
            sub_tokens.push_back(current);
        }
    }

    Ok(params)
}
