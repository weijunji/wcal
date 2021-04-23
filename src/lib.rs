//! A calculator that implement for Arithmetic.
//! 
//! Allow operator: `+` `-` `*` `/` `(` `)`.
//!
//! Result can be `i128` or `f64`. A warning will
//! occur while result is `i128` and division cast
//! happened, such as `3/2=1`.
//!
//! This calculator has three steps:
//! * Use `logos` to parse the expression to tokens.
//! * Use a parser to parse tokens to a AST.
//! * Calculate the result from the AST.
//!
//! The following parser is available:
//! * Top-down parser (default)
//! 
//! # Example
//! ```
//! use wcal::{calculator, parser};
//! 
//! fn main() {
//!     let res: f64 = calculator!("1+2").unwrap();
//!     assert_eq!(res, 3f64);
//! 
//!     let res: i128 = calculator("1+2", wcal::parser::top_down_parser::parse).unwrap();
//!     assert_eq!(res, 3);
//! 
//!     let res: f64 = calculator("1+2", wcal::parser::top_down_parser::parse).unwrap();
//!     assert_eq!(res, 3f64);
//! }
//! ```
pub mod lexer;
pub mod parser;
pub mod generator;

use parser::ast::AST;
use generator::{calculator, calculator_f};

/// Use default parser to calculate the expression.
#[macro_export]
macro_rules! calculator{
    ($expr: expr) => {
        calculator($expr, parser::top_down_parser::parse);
    };
    ($expr: expr, $type: ty) => {
        calculator::<$type>($expr, parser::top_down_parser::parse);
    }
}

/// Result that can be calculate from the AST
pub trait FromAST{
    fn from_ast(ast: AST) -> Self;
}

impl FromAST for i128 {
    fn from_ast(ast: AST) -> i128 {
        calculator::calculate(ast)
    }
}

impl FromAST for f64 {
    fn from_ast(ast: AST) -> f64 {
        calculator_f::calculate(ast)
    }
}

/// Use a parser to calculate the expression.
pub fn calculator<T: FromAST>(expr: &str, parser: fn(Vec<lexer::Token>)->Result<AST, String>) -> Result<T, String> {
    let tokens = lexer::lexer(expr)?;
    let ast = parser(tokens)?;
    Ok(T::from_ast(ast))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cal() -> Result<(), String> {
        let res: i128 = calculator("1+2", parser::top_down_parser::parse)?;
        assert_eq!(res, 3);

        let res: f64 = calculator("1+2", parser::top_down_parser::parse)?;
        assert_eq!(res, 3f64);

        let res = calculator::<f64>("1+2", parser::top_down_parser::parse)?;
        assert_eq!(res, 3f64);
        Ok(())
    }

    #[test]
    fn test_cal_macro() -> Result<(), String> {
        let res: f64 = calculator!("1+2")?;
        assert_eq!(res, 3f64);

        let res: f64 = calculator!("1+2", f64)?;
        assert_eq!(res, 3f64);
        Ok(())
    }
}
