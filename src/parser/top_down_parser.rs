//! Use top-down method to parse.
//!
//! Convert the grammar to the following grammar
//! to solve priority:
//! ```text
//! S ::= expr
//!
//! <expr> ::= <term> <expr_tail>
//! <expr_tail> ::= Plus <term> <expr_tail>
//!               | Minus <term> <expr_tail>
//!               | <empty>
//!
//! <term> ::= <factor> <term_tail>
//! <term_tail> ::= Times <factor> <term_tail>
//!               | Division <factor> <term_tail>
//!               | <empty>
//!
//! <factor> ::= LP <expr> RP
//!          | Number
//!          | Minus <factor>
//! ```
use crate::lexer::Token;

use std::iter::Peekable;
use std::slice::Iter;

trait Calculable {
    fn calculate_f(&self) -> f64;
    fn calculate(&self) -> i128;
}

/// ( expr )
struct Pair{
    expr: Box<dyn Calculable>
}

impl Calculable for Pair{
    fn calculate_f(&self) -> f64 {
        self.expr.calculate_f()
    }

    fn calculate(&self) -> i128 {
        self.expr.calculate()
    }
}

/// lhs op rhs
///
/// op is `+` `-` `*` or `/`
struct Expr{
    lhs: Box<dyn Calculable>,
    rhs: Box<dyn Calculable>,
    op: Token
}

impl Calculable for Expr{
    fn calculate_f(&self) -> f64 {
        let lval = self.lhs.calculate_f();
        let rval = self.rhs.calculate_f();
        match self.op {
            Token::Plus => lval + rval,
            Token::Minus => lval - rval,
            Token::Times => lval * rval,
            Token::Division => lval / rval,
            _ => panic!("Unknown operator")
        }
    }

    fn calculate(&self) -> i128 {
        let lval = self.lhs.calculate();
        let rval = self.rhs.calculate();
        match self.op {
            Token::Plus => lval + rval,
            Token::Minus => lval - rval,
            Token::Times => lval * rval,
            Token::Division => {
                if rval == 0 {
                    println!("Error: division by zero");
                    panic!()
                }
                if lval % rval != 0 {
                    println!("Warning: division will cause a cast");
                }
                lval / rval
            }
            _ => panic!("Unknown operator")
        }
    }
}

/// -expr
struct Neg{
    expr: Box<dyn Calculable>
}

impl Calculable for Neg{
    fn calculate_f(&self) -> f64 {
        -self.expr.calculate_f()
    }

    fn calculate(&self) -> i128 {
        -self.expr.calculate()
    }
}

/// number store as `u64`
struct Number{
    num: u64
}

impl Calculable for Number{
    fn calculate_f(&self) -> f64 {
        self.num as f64
    }

    fn calculate(&self) -> i128 {
        self.num as i128
    }
}

pub struct AST{
    root: Box<dyn Calculable>
}

impl AST{
    pub fn calculate_f(&self) -> f64 {
        self.root.calculate_f()
    }

    pub fn calculate(&self) -> i128 {
        self.root.calculate()
    }
}

struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>
}

impl<'a> Parser<'a> {
    fn eof(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    fn s(&mut self) -> Result<Box<dyn Calculable>, String> {
        self.expr()
    }

    fn expr(&mut self) -> Result<Box<dyn Calculable>, String> {
        let lhs = self.term()?;
        self.expr_tail(lhs)
    }

    fn expr_tail(&mut self, lhs: Box<dyn Calculable>) -> Result<Box<dyn Calculable>, String> {
        let token = self.iter.peek();
        match token {
            Some(Token::Plus) => {
                self.get_token("+")?;
                let rhs = self.term()?;
                self.expr_tail(Box::new(Expr{lhs, rhs, op: Token::Plus}))
            }
            Some(Token::Minus) => {
                self.get_token("-")?;
                let rhs = self.term()?;
                self.expr_tail(Box::new(Expr{lhs, rhs, op: Token::Minus}))
            }
            _ => {
                Ok(lhs)
            }
        }
    }

    fn term(&mut self) -> Result<Box<dyn Calculable>, String> {
        let lval = self.factor()?;
        self.term_tail(lval)
    }

    fn term_tail(&mut self, lhs: Box<dyn Calculable>) -> Result<Box<dyn Calculable>, String> {
        let token = self.iter.peek();
        match token {
            Some(Token::Times) => {
                self.get_token("*")?;
                let rhs = self.factor()?;
                self.term_tail(Box::new(Expr{lhs, rhs, op: Token::Times}))
            }
            Some(Token::Division) => {
                self.get_token("/")?;
                let rhs = self.factor()?;
                self.term_tail(Box::new(Expr{lhs, rhs, op: Token::Division}))
            }
            _ => {
                Ok(lhs)
            }
        }
    }

    fn factor(&mut self) -> Result<Box<dyn Calculable>, String> {
        let token = self.get_token("number")?;
        match token {
            Token::LP => {
                let expr = self.expr()?;
                self.get_token(")")?;
                Ok(Box::new(Pair{expr}))
            }
            Token::Minus => {
                let expr = self.factor()?;
                Ok(Box::new(Neg{expr}))
            }
            Token::Number(num) => {
                Ok(Box::new(Number{num}))
            }
            _ => {
                Err(format!("Expect number, got {}", token))
            }
        }
    }

    fn get_token(&mut self, expect: &str) -> Result<Token, String> {
        if let Some(token) = self.iter.next() {
            Ok(*token)
        } else {
            Err(format!("Expect {}, got nothing", expect))
        }
    }
}

/// Parse tokens to AST.
///
/// # Example
/// ```
/// use wcal::lexer;
/// use wcal::parser::top_down_parser::parse;
///
/// let tokens = lexer::lexer("12+3*4/2- 2+-1").unwrap();
/// let ast = parse(tokens).unwrap();
/// assert_eq!(ast.calculate_f(), 15f64);
/// assert_eq!(ast.calculate(), 15i128);
/// ```
pub fn parse(tokens: Vec<Token>) -> Result<AST, String> {
    let mut parser = Parser{
        iter: tokens.iter().peekable()
    };
    let root = parser.s()?;
    if parser.eof() {
        Ok(AST{root})
    } else {
        Err(String::from("Invalid expression"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_add() -> Result<(), String> {
        let tokens = lexer::lexer("12+3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 15f64);
        assert_eq!(ast.calculate(), 15i128);
        Ok(())
    }

    #[test]
    fn test_sub() -> Result<(), String> {
        let tokens = lexer::lexer("12-3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 9f64);
        assert_eq!(ast.calculate(), 9i128);
        Ok(())
    }

    #[test]
    fn test_times() -> Result<(), String> {
        let tokens = lexer::lexer("12*3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 36f64);
        assert_eq!(ast.calculate(), 36i128);
        Ok(())
    }

    #[test]
    fn test_div() -> Result<(), String> {
        let tokens = lexer::lexer("12/3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 4f64);
        assert_eq!(ast.calculate(), 4i128);
        Ok(())
    }

    #[test]
    fn test_div_cast() -> Result<(), String> {
        let tokens = lexer::lexer("7/2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 3.5f64);
        assert_eq!(ast.calculate(), 3i128);
        Ok(())
    }

    #[test]
    fn test_num() -> Result<(), String> {
        let tokens = lexer::lexer("12")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 12f64);
        assert_eq!(ast.calculate(), 12i128);

        let tokens = lexer::lexer("-12")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), -12f64);
        assert_eq!(ast.calculate(), -12i128);
        Ok(())
    }

    #[test]
    fn test_pair() -> Result<(), String> {
        let tokens = lexer::lexer("(12)")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 12f64);
        assert_eq!(ast.calculate(), 12i128);

        let tokens = lexer::lexer("(((((((12))))))+(((1))))")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 13f64);
        assert_eq!(ast.calculate(), 13i128);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_div_zero(){
        let tokens = lexer::lexer("5/0").unwrap();
        let ast = parse(tokens).unwrap();
        ast.calculate();
    }

    #[test]
    fn test_div_zero_f() -> Result<(), String> {
        let tokens = lexer::lexer("5/0")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), f64::INFINITY);
        Ok(())
    }

    #[test]
    fn test_neg() -> Result<(), String> {
        let tokens = lexer::lexer("-------7-------2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), -9f64);
        assert_eq!(ast.calculate(), -9i128);

        let tokens = lexer::lexer("-------7------2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), -5f64);
        assert_eq!(ast.calculate(), -5i128);

        let tokens = lexer::lexer("------7------2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), 9f64);
        assert_eq!(ast.calculate(), 9i128);

        let tokens = lexer::lexer("-(1+2)")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate_f(), -3f64);
        assert_eq!(ast.calculate(), -3i128);

        Ok(())
    }

    #[test]
    fn test_priority() -> Result<(), String> {
        let tokens = lexer::lexer("1+3*6/2-3*-1")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate(), 13i128);

        let tokens = lexer::lexer("1+3*6/(2-3)*-1")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate(), 19i128);

        Ok(())
    }

    #[test]
    fn test_expect_num() {
        let tokens = lexer::lexer("1+").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Expect number, got nothing");

        let tokens = lexer::lexer("+").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Expect number, got +");

        let tokens = lexer::lexer("(").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Expect number, got nothing");
    }

    #[test]
    fn test_pair_error() {
        let tokens = lexer::lexer("(((2))").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Expect ), got nothing");

        let tokens = lexer::lexer("(2)(1)").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Invalid expression");

        let tokens = lexer::lexer("(())").unwrap();
        let err = parse(tokens).err().unwrap();
        assert_eq!(err, "Expect number, got )");
    }

    #[test]
    fn test_one_line() -> Result<(), String> {
        let tokens = lexer::lexer("1+3*6/2-3*-1\n-1*2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast.calculate(), 13i128);
        Ok(())
    }
}
