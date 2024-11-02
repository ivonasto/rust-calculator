use std::collections::VecDeque;
use super::{error::CustomError,token_type::TokenType};

pub fn interpret(mut tokens: VecDeque<TokenType>)-> Result<f64,CustomError> {

    let mut operands: Vec<f64> = Vec::new();
    while let Some(t) = tokens.pop_front(){
        match t { 
        TokenType::NUMBER(value) =>  operands.push(value),
        TokenType::ADD => { 
            if let (Some(a), Some(b)) = (operands.pop(), operands.pop()) {
            operands.push(a + b);}},
        TokenType::MUL => {
            if let (Some(a), Some(b)) = (operands.pop(), operands.pop()) {
                operands.push(a * b);}
        },
        TokenType::SUB => {
            if let (Some(a), Some(b)) = (operands.pop(), operands.pop()) {
                operands.push(b - a);}    
        },
        TokenType::DIV => {
            if let (Some(a), Some(b)) = (operands.pop(), operands.pop()) {
                if b==0.0 {
                return Err(CustomError::DivisionByZero);
                }
                operands.push(b / a);}        
        },
        TokenType::LEFT_PAREN | TokenType::RIGHT_PAREN => return Err(CustomError::MismatchedParentheses),
    }
}
    let operand = operands.pop().ok_or_else(|| CustomError::NotEnoughOperands)?;
    if operands.is_empty(){
        return Ok(operand)
        } else {return Err(CustomError::NotEnoughOperators)};
   
}