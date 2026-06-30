//! A small arithmetic library used for learning Cargo and crates.io publishing.
//!
//! ```
//! use lwmacct_260629_rust_helloworld::{add, divide};
//!
//! assert_eq!(add(1.0, 2.0), 3.0);
//! assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
//! ```

use std::error::Error;
use std::fmt;

/// The arithmetic operation to perform with [`calculate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Add two numbers.
    Add,
    /// Subtract the right number from the left number.
    Subtract,
    /// Multiply two numbers.
    Multiply,
    /// Divide the left number by the right number.
    Divide,
}

/// Errors returned by checked arithmetic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    /// Division by zero is not allowed.
    DivisionByZero,
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl Error for ArithmeticError {}

/// Adds two numbers.
pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

/// Subtracts the right number from the left number.
pub fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

/// Multiplies two numbers.
pub fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

/// Divides the left number by the right number.
pub fn divide(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        Err(ArithmeticError::DivisionByZero)
    } else {
        Ok(left / right)
    }
}

/// Calculates an arithmetic operation for two numbers.
pub fn calculate(operation: Operation, left: f64, right: f64) -> Result<f64, ArithmeticError> {
    match operation {
        Operation::Add => Ok(add(left, right)),
        Operation::Subtract => Ok(subtract(left, right)),
        Operation::Multiply => Ok(multiply(left, right)),
        Operation::Divide => divide(left, right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_basic_operations() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(subtract(2.0, 3.0), -1.0);
        assert_eq!(multiply(2.0, 3.0), 6.0);
        assert_eq!(divide(6.0, 3.0), Ok(2.0));
    }

    #[test]
    fn rejects_division_by_zero() {
        assert_eq!(divide(1.0, 0.0), Err(ArithmeticError::DivisionByZero));
    }

    #[test]
    fn calculates_with_operation_enum() {
        assert_eq!(calculate(Operation::Add, 1.5, 2.0), Ok(3.5));
        assert_eq!(calculate(Operation::Subtract, 1.5, 2.0), Ok(-0.5));
        assert_eq!(calculate(Operation::Multiply, 1.5, 2.0), Ok(3.0));
        assert_eq!(calculate(Operation::Divide, 3.0, 2.0), Ok(1.5));
    }
}
