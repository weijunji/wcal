//! Convert the expression AST to `i128`
//!
//! A warning will raise while division cast happened
//!
//! A error will raise while division by zero
use crate::lexer::Token;
use crate::parser::ast::*;

trait Calculable {
    fn calculate(node: &Self) -> i128;
}

impl Calculable for Expr {
    fn calculate(node: &Self) -> i128 {
        match node {
            Expr::Pair(pair) => Pair::calculate(pair),
            Expr::BinOp(expr) => BinOp::calculate(expr),
            Expr::Neg(neg) => Neg::calculate(neg),
            Expr::Num(num) => Number::calculate(num),
        }
    }
}

impl Calculable for BinOp {
    fn calculate(node: &Self) -> i128 {
        let lval = Expr::calculate(&node.lhs);
        let rval = Expr::calculate(&node.rhs);
        match node.op {
            Token::Plus => lval + rval,
            Token::Minus => lval - rval,
            Token::Times => lval * rval,
            Token::Division => {
                if rval == 0 {
                    eprintln!("Error: division by zero");
                    panic!()
                }
                if lval % rval != 0 {
                    eprintln!("Warning: division will cause a cast");
                }
                lval / rval
            },
            _ => panic!("Unknown operator")
        }
    }
}

impl Calculable for Number {
    fn calculate(node: &Self) -> i128 {
        node.num as i128
    }
}

impl Calculable for Pair {
    fn calculate(node: &Self) -> i128 {
        Expr::calculate(&node.expr)
    }
}

impl Calculable for Neg {
    fn calculate(node: &Self) -> i128 {
        -Expr::calculate(&node.expr)
    }
}

impl Calculable for AST {
    fn calculate(ast: &Self) -> i128 {
        Expr::calculate(&ast.root)
    }
}

/// Calculate the expression's AST to `i28`
pub fn calculate(ast: AST) -> i128 {
    AST::calculate(&ast)
}


#[cfg(test)]
mod tests {
    use crate::generator::calculator;
    use crate::parser::ast::*;
    use crate::lexer::Token;

    #[test]
    fn test_num() {
        let res = calculator::calculate(AST{root: Number::new(3)});
        assert_eq!(res, 3);
    }

    #[test]
    fn test_add() {
        let res = calculator::calculate(AST{root: BinOp::new(Number::new(1), Number::new(2), Token::Plus)});
        assert_eq!(res, 3);
    }

    #[test]
    fn test_minus() {
        let res = calculator::calculate(AST{root: BinOp::new(Number::new(1), Number::new(2), Token::Minus)});
        assert_eq!(res, -1);
    }

    #[test]
    fn test_times() {
        let res = calculator::calculate(AST{root: BinOp::new(Number::new(1), Number::new(2), Token::Times)});
        assert_eq!(res, 2);
    }

    #[test]
    fn test_division() {
        let res = calculator::calculate(AST{root: BinOp::new(Number::new(4), Number::new(2), Token::Division)});
        assert_eq!(res, 2);
    }

    #[test]
    fn test_division_cast() {
        let res = calculator::calculate(AST{root: BinOp::new(Number::new(3), Number::new(2), Token::Division)});
        assert_eq!(res, 1);
    }

    #[test]
    #[should_panic]
    fn test_division_zero() {
        calculator::calculate(AST{root: BinOp::new(Number::new(3), Number::new(0), Token::Division)});
    }

    #[test]
    fn test_neg() {
        // -3
        let res = calculator::calculate(AST{root: Neg::new(Number::new(3))});
        assert_eq!(res, -3);
        // --3
        let res = calculator::calculate(AST{root: Neg::new(Neg::new(Number::new(3)))});
        assert_eq!(res, 3);
        // --3---3
        let res = calculator::calculate(AST{root: BinOp::new(
            Neg::new(Neg::new(Number::new(3))),
            Neg::new(Neg::new(Number::new(3))),
            Token::Minus
        )});
        assert_eq!(res, 0);
    }

    #[test]
    fn test_pair() {
        // (3)
        let res = calculator::calculate(AST{root: Pair::new(Number::new(3))});
        assert_eq!(res, 3);
        // ((3))
        let res = calculator::calculate(AST{root: Pair::new(Pair::new(Number::new(3)))});
        assert_eq!(res, 3);
    }
}
