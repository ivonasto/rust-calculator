use super::error:: CustomError;
use super::token_type::TokenType;
use std::collections::VecDeque;

pub fn scan(input: &String) -> Result<VecDeque<TokenType>, CustomError> {
    let mut tokens: VecDeque<TokenType> = VecDeque::new();

    let mut iter = input.trim().chars().peekable();
    // go through it
    while let Some(c) = iter.next() {
        match c {
            ' ' => continue,
            '+' => tokens.push_back(TokenType::ADD),
            '-' => tokens.push_back(TokenType::SUB),
            '/' => tokens.push_back(TokenType::DIV),
            '*' => tokens.push_back(TokenType::MUL),
            '(' => tokens.push_back(TokenType::LEFT_PAREN),
            ')' => tokens.push_back(TokenType::RIGHT_PAREN),
            '0'..='9' => {
                extract_number(c, &mut iter, &mut tokens);
            }
            err => {
                return Err(CustomError::UnknownOperator(err))
            }
        }
    }
    Ok(tokens)
}

fn extract_number(
    c: char,
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    tokens: &mut VecDeque<TokenType>,
) {
    let mut number = String::new();
    number.push(c);
    while let Some(&ch) = iter.peek() {
        if ch >= '0' && ch <= '9' {
            number.push(ch);
            iter.next();
        } else {
            break;
        };
    }
    tokens.push_back(TokenType::NUMBER(number.parse::<f64>().unwrap().into()))
}
