#![allow(non_camel_case_types)]
#[derive(Debug)]

pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    NUMBER(f64),
    ADD,
    SUB,
    MUL,
    DIV
}

pub fn precedence(t: &TokenType) -> u8 {
    match t {
        TokenType::ADD | TokenType::SUB => 1,
        TokenType::MUL | TokenType::DIV => 2,
        _ => 0,
    }
}
