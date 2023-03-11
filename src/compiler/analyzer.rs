use crate::public::token::{TokenTypes, TokenVec};
use crate::public::ast::{ASTNode, ASTNodeVec, ASTNodeTypes};
use crate::public::symbols::Symbols;

// input default with paired-paren
fn expression_resolve(
    tokens: &mut TokenVec,
    within_paren: bool,
) -> Result<ASTNodeVec, ()> {
    if tokens.len() == 0 {
        println!("Empty expression.");
        return Err(())
    }

    let mut params = ASTNodeVec::new();
    let first_index = 0;

    if within_paren {
        // if `within_paren`, first token should be Symbols::LeftParen
        if tokens[first_index].symbol != Symbols::LeftParen {
            println!("Analyzer error.");
            return Err(())
        }
        tokens.remove(first_index);
    }

    while first_index < tokens.len() {
        let current = &tokens[first_index];

        match current.type__ {
            TokenTypes::Unknown => {},
            TokenTypes::Number => params.push(ASTNode {
                type__: ASTNodeTypes::NumberLiteral(
                    tokens.remove(first_index).number
                ),
                params: None,
            }),
            TokenTypes::Symbol => {
                if current.symbol == Symbols::Equal {
                    println!("Invalid variable / goto-statement assignment.");
                    return Err(())
                }
                params.push(ASTNode {
                    type__: ASTNodeTypes::SymbolLiteral(
                        tokens.remove(first_index).symbol
                    ),
                    params: None,
                })
            },
            TokenTypes::Identifier => {
                // identi should only be variable and
                // function invocation
                let identi = tokens
                    .remove(first_index)
                    .identi
                    .unwrap();

                // if there is the next token
                // and the next token if LeftParen.
                if tokens.len() > 0 &&
                   tokens[first_index].symbol == Symbols::LeftParen {
                    // function invocation
                    let mut current_node = ASTNode {
                        type__: ASTNodeTypes::InvokeExpression(identi),
                        params: None,
                    };
                    // function params
                    let expression_ast = expression_resolve(tokens, true)?;
                    current_node.params = Some(expression_ast);
                    params.push(current_node);
                } else {
                    // variable reading
                    params.push(ASTNode {
                        type__: ASTNodeTypes::Variable(identi),
                        params: None,
                    });
                }
            },
            TokenTypes::Paren => {
                if current.symbol == Symbols::LeftBrace {
                    // Goto-statement
                    // vec[expression-node]

                    // remove the LeftBrace
                    tokens.remove(first_index);

                    let mut sub_tokens = TokenVec::new();
                    while first_index < tokens.len() {
                        if first_index == tokens.len() {
                            println!("Unmatched brace");
                            return Err(())
                        }

                        let current = &tokens[first_index];
                        if current.symbol == Symbols::RightBrace {
                            tokens.remove(first_index);
                            break;
                        }
                        sub_tokens.push(tokens.remove(first_index));
                    }
                    let expression_params = expression_resolve(&mut sub_tokens, false)?;
                    let expression_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(expression_params),
                    };
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::GotoStatement,
                        params: Some(vec![expression_node]),
                    };

                    params.push(current_node);
                } else
                if current.symbol == Symbols::LeftParen {
                    // nested expression
                    let nested_expression = expression_resolve(tokens, true)?;
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(nested_expression),
                    };
                    params.push(current_node);
                } else
                if current.symbol == Symbols::RightParen {
                    // remove the RightParen from the tokens
                    tokens.remove(first_index);
                    break;
                }
            },
        }
    }
    Ok(params)
}

pub fn analyzer(mut tokens: TokenVec) -> Result<ASTNode, ()> {
    if tokens.len() == 0 {return Err(())}

    let mut root = ASTNode {
        type__: ASTNodeTypes::Root,
        params: None,
    };
    let mut params = ASTNodeVec::new();

    let first_token = &tokens[0];
    // identi + Equal
    if first_token.type__ == TokenTypes::Identifier
       && 1 < tokens.len() &&
       tokens[1].symbol == Symbols::Equal {
        // assignment statement

        // get the identifier and remove
        // the two element at the first.
        let identi = tokens
            .remove(0)
            .identi
            .unwrap();
        tokens.remove(0);

        // examples for right-hand expression:
        // 1 + 1     |  from: var = 1 + 1
        // {var + 1} |  from: func = {var + 1}
        let right_hand_nodes = expression_resolve(&mut tokens, false)?;

        let right_hand_expression = if right_hand_nodes[0].type__ == ASTNodeTypes::GotoStatement {
            right_hand_nodes[0].to_owned()
        } else {
            ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(right_hand_nodes)
            }
        };

        let current_node = ASTNode {
            type__: ASTNodeTypes::Assignment(identi),
            params: Some(vec![right_hand_expression]),
        };

        params.push(current_node);
    } else {
        // regard the whole statement as a expression
        let expression_ast = expression_resolve(&mut tokens, false)?;
        let current_node = ASTNode {
            type__: ASTNodeTypes::Expression,
            params: Some(expression_ast),
        };

        params.push(current_node);
    }
    root.params = Some(params);

    Ok(root)
}