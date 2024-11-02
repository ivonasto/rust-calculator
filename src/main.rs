mod calculator;

use calculator::parser::shunting_yard as parse;
use crate::calculator::{interpreter::interpret, scanner::scan,error::CustomError};
use std::io::{self, Write};

fn main() {
    run_prompt();
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,

            Ok(_n) => match run(input)  {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
            Err(error) => println!("Error: {:?}", error)
        };
    }

}

fn run(line: String) -> Result<f64, CustomError> {
    let tokens = scan(&line)?;             
    let tokens = parse(tokens)?; 
    let result = interpret(tokens)?;      
    Ok(result)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(run("2 + 2".to_string()), Ok(4.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(run("2 * 2".to_string()), Ok(4.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(run("2 / 2".to_string()), Ok(1.0));
    }

    #[test]
    fn test_substraction() {
        assert_eq!(run("2 - 2".to_string()), Ok(0.0));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        assert_eq!(run("2 / 0 ".to_string()), Err(CustomError::DivisionByZero));
    }

    #[test]
    fn test_unclosed_parentheses_should_err() {
        assert_eq!(run("2 + )".to_string()), Err(CustomError::MismatchedParentheses));
    }

    #[test]
    fn test_unclosed_parentheses_should_err2() {
        assert_eq!(run("2 + (".to_string()), Err(CustomError::MismatchedParentheses));
    }

    #[test]
    fn unknown_operators_should_err(){
        assert_eq!(run("2 & 2".to_string()), Err(CustomError::UnknownOperator('&')));
    }

    #[test]
    fn incomplete_equation_should_err(){
        assert_eq!(run("2 +".to_string()), Err(CustomError::NotEnoughOperands));
    }

    #[test]
    fn incomplete_equation_should_err2(){
        assert_eq!(run("2 2".to_string()), Err(CustomError::NotEnoughOperators));
    }

}