use super::{error::CustomError, token_type::*};
use std::collections::VecDeque;

pub fn shunting_yard(mut tokens: VecDeque<TokenType>) -> Result<VecDeque<TokenType>,CustomError> {
    let mut output: VecDeque<TokenType> = VecDeque::new();
    let mut operators: Vec<TokenType> = Vec::new();

    while let Some(token) = tokens.pop_front()  {
        match token {
            TokenType::NUMBER(_) => output.push_back(token),
            TokenType::LEFT_PAREN => operators.push(token),
            TokenType::RIGHT_PAREN =>
           {
                loop {
                    match operators.pop() {
                        Some(TokenType::LEFT_PAREN) => break,
                        Some(token) => output.push_back(token),
                        None => return Err(CustomError::MismatchedParentheses),
                    }
                }
            }
            _ => handle_operator(token, &mut output, &mut operators),
        }
    }

    while let Some(operator) = operators.pop(){
        output.push_back(operator);
    }

    Ok(output)
}

fn handle_operator(
    token: TokenType,
    output: &mut VecDeque<TokenType>,
    operators: &mut Vec<TokenType>,
) {
    while let Some(operator) = operators.last_mut() {
        if precedence(&operator) >= precedence(&token) {
            output.push_back(operators.pop().expect("I miscalculated"));
        } else {
            break;
        }
    }

    operators.push(token);
}
