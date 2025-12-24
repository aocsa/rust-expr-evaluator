use crate::ast::Expr;
use crate::error::{LexerError, Location, ParseError};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    current_location: Location,
}

// expr       → term (('+' | '-') term)*
// term       → unary (('*' | '/') unary)*
// unary      → '-' unary | primary
// primary    → NUMBER | '(' expr ')'

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, LexerError> {
        let mut lexer = Lexer::new(input);
        let location = lexer.location();
        let current = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current,
            current_location: location,
        })
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let prev = self.current.clone();
        self.current_location = self.lexer.location();
        self.current = self.lexer.next_token()?;
        Ok(prev)
    }

    fn check(&self, expected: &Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(expected)
    }

    fn expect_and_advance(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.check(&expected) {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::new(
                format!("Expected {:?}, got {:?}", expected, self.current),
                self.current_location,
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        if self.current != Token::Eof {
            return Err(ParseError::new(
                format!("Expected end of input, got {:?}", self.current),
                self.current_location,
            ));
        }
        Ok(expr)
    }

    // expr → term (('+' | '-') term)*
    fn expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.term()?;

        loop {
            match &self.current {
                Token::Plus => {
                    self.advance()?;
                    let right = self.term()?;
                    left = Expr::add(left, right);
                }
                Token::Minus => {
                    self.advance()?;
                    let right = self.term()?;
                    left = Expr::sub(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // term       → unary (('*' | '/') unary)*
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut unary = self.unary()?;
        loop {
            match &self.current {
                Token::Star => {
                    self.advance()?;
                    unary = Expr::mul(unary, self.unary()?);
                }
                Token::Slash => {
                    self.advance()?;
                    unary = Expr::div(unary, self.unary()?);
                }
                _ => break,
            }
        }
        Ok(unary)
    }

    // unary      → '-' unary | primary
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.current == Token::Minus {
            self.advance()?;
            let expr = self.unary()?;
            return Ok(Expr::neg(expr));
        }
        self.primary()
    }

    // primary    → NUMBER | '(' expr ')'
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.current {
            Token::Number(n) => {
                self.advance()?;
                Ok(Expr::number(n))
            }
            Token::LeftParen => {
                self.advance()?;
                let expr = self.expression()?;
                self.expect_and_advance(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::new(
                format!("Expected expression, got {:?}", self.current),
                self.current_location,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let inputs = [
            "2 + 3",
            "2 + 3 * 4",
            "10 - 2 - 3",
            "(2 + 3) * 4",
            "-5",
            "--5",
            "2 * -3",
        ];
        for input in inputs {
            let mut parser = Parser::new(input).expect("Failed to create parser");
            match parser.parse() {
                Ok(expr) => {
                    let result = expr.eval();
                    match result {
                        Ok(value) => println!("expr: {:?} = {:?}", expr.to_string(), value),
                        Err(e) => println!("eval error: {}", e),
                    }
                }
                Err(e) => println!("parse error: {}", e),
            }
        }
    }

    #[test]
    fn test_parser_error_location() {
        // "2 + + 3" - error at second '+' which is at column 5
        // When we reach the second '+', it's an unexpected token
        // The lexer reports position after skipping whitespace
        let result = Parser::new("2 ++ 3");
        assert!(result.is_ok());
        let mut parser = result.unwrap();
        let err = parser.parse();
        assert!(err.is_err());
        let err = err.unwrap_err();
        // After "2 +", the next token starts at column 4 (the second '+')
        assert_eq!(err.location.column, 4);
    }

    #[test]
    fn test_parser_unbalanced_paren() {
        let mut parser = Parser::new("(2 + 3").expect("Failed to create parser");
        let err = parser.parse();
        assert!(err.is_err());
        let err = err.unwrap_err();
        assert!(err.message.contains("Expected RightParen"));
    }

    #[test]
    fn test_parser_lexer_error() {
        let mut parser = Parser::new("2 + @").expect("Failed to create parser");
        let err = parser.parse();
        assert!(err.is_err());
    }
}
