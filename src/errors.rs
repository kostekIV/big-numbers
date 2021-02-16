use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ArithmeticError {
    DividedByZero,
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ArithmeticError::DividedByZero => write!(f, "Division by zero"),
        }
    }
}
