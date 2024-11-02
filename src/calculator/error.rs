#[derive(Debug,PartialEq)]
pub enum CustomError {
    DivisionByZero,
    UnknownOperator(char),
    NotEnoughOperators,
    NotEnoughOperands,
    MismatchedParentheses,

}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
            Self::NotEnoughOperands =>write!(f, "Please provide enough operands"),
            Self::NotEnoughOperators => write!(f, "Not enough operators"),
            Self::MismatchedParentheses => write!(f, "Mismatched parentheses"),
        }
    }
}

impl std::error::Error for CustomError {}