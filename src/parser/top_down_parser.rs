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
//!            | Number
//!            | Minus <factor>
//! ```
use crate::lexer::Token;
use super::ast::*;

use std::iter::Peekable;
use std::slice::Iter;


struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>
}

impl<'a> Parser<'a> {
    fn eof(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    fn s(&mut self) -> Result<Expr, String> {
        self.expr()
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let lhs = self.term()?;
        self.expr_tail(lhs)
    }

    fn expr_tail(&mut self, lhs: Expr) -> Result<Expr, String> {
        let token = self.iter.peek();
        match token {
            Some(Token::Plus) => {
                self.get_token("+")?;
                let rhs = self.term()?;
                self.expr_tail(BinOp::new(lhs, rhs, Token::Plus))
            }
            Some(Token::Minus) => {
                self.get_token("-")?;
                let rhs = self.term()?;
                self.expr_tail(BinOp::new(lhs, rhs, Token::Minus))
            }
            _ => {
                Ok(lhs)
            }
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
        let lval = self.factor()?;
        self.term_tail(lval)
    }

    fn term_tail(&mut self, lhs: Expr) -> Result<Expr, String> {
        let token = self.iter.peek();
        match token {
            Some(Token::Times) => {
                self.get_token("*")?;
                let rhs = self.factor()?;
                self.term_tail(BinOp::new(lhs, rhs, Token::Times))
            }
            Some(Token::Division) => {
                self.get_token("/")?;
                let rhs = self.factor()?;
                self.term_tail(BinOp::new(lhs, rhs, Token::Division))
            }
            _ => {
                Ok(lhs)
            }
        }
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let token = self.get_token("number")?;
        match token {
            Token::LP => {
                let expr = self.expr()?;
                self.get_token(")")?;
                Ok(Pair::new(expr))
            }
            Token::Minus => {
                let expr = self.factor()?;
                Ok(Neg::new(expr))
            }
            Token::Number(num) => {
                Ok(Number::new(num))
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
/// use wcal::parser::ast::*;
///
/// let tokens = lexer::lexer("12+3").unwrap();
/// let ast = parse(tokens).unwrap();
/// assert_eq!(ast, AST{root: BinOp::new(Number::new(12), Number::new(3), lexer::Token::Plus)});
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
        assert_eq!(ast, AST{root: BinOp::new(Number::new(12), Number::new(3), lexer::Token::Plus)});
        Ok(())
    }

    #[test]
    fn test_sub() -> Result<(), String> {
        let tokens = lexer::lexer("12-3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Number::new(12), Number::new(3), lexer::Token::Minus)});
        Ok(())
    }

    #[test]
    fn test_times() -> Result<(), String> {
        let tokens = lexer::lexer("12*3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Number::new(12), Number::new(3), lexer::Token::Times)});
        Ok(())
    }

    #[test]
    fn test_div() -> Result<(), String> {
        let tokens = lexer::lexer("12/3")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Number::new(12), Number::new(3), lexer::Token::Division)});
        Ok(())
    }


    #[test]
    fn test_num() -> Result<(), String> {
        let tokens = lexer::lexer("12")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: Number::new(12)});

        let tokens = lexer::lexer("-12")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: Neg::new(Number::new(12))});
        Ok(())
    }

    #[test]
    fn test_pair() -> Result<(), String> {
        let tokens = lexer::lexer("((12))")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: Pair::new(Pair::new(Number::new(12)))});
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
    fn test_priority() -> Result<(), String> {
        let tokens = lexer::lexer("1+3*6")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Number::new(1), BinOp::new(Number::new(3), Number::new(6), lexer::Token::Times), lexer::Token::Plus)});

        let tokens = lexer::lexer("6/(2-3)")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Number::new(6), Pair::new(BinOp::new(Number::new(2), Number::new(3), lexer::Token::Minus)), lexer::Token::Division)});

        Ok(())
    }

    #[test]
    fn test_neg() -> Result<(), String> {
        let tokens = lexer::lexer("-7--2")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: BinOp::new(Neg::new(Number::new(7)), Neg::new(Number::new(2)), lexer::Token::Minus)});

        let tokens = lexer::lexer("---7")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: Neg::new(Neg::new(Neg::new(Number::new(7))))});

        let tokens = lexer::lexer("-(1+2)")?;
        let ast = parse(tokens)?;
        assert_eq!(ast, AST{root: Neg::new(Pair::new(BinOp::new(Number::new(1), Number::new(2), lexer::Token::Plus)))});

        Ok(())
    }
}
