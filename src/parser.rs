use crate::ast::Expr;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        ParseError { message: message.into() }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

// expr       → term (('+' | '-') term)*
// term       → unary (('*' | '/') unary)*
// unary      → '-' unary | primary
// primary    → NUMBER | '(' expr ')'


impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Parser {
            lexer: lexer,
            current: current,
        }
    }

    fn advance(&mut self) -> Token {
        // let prev = std::mem::replace(&mut self.current, self.lexer.next_token());
        let prev = self.current.clone();
        self.current = self.lexer.next_token();
        prev
    }

    fn check(&self, expected: &Token) -> bool  {
        // self.current == *expected; // does not work because Number(1.0) == Number(5.0) is false, I just want to compare the tags
        // matches!(self.current, Token::Number(_)) // onlt for numbers
        std::mem::discriminant(&self.current) == std::mem::discriminant(expected)
    }

    fn expect_and_advance(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(format!("Expected {:?}, got {:?}", expected, self.current)))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        if self.current != Token::Eof {
            return Err(ParseError::new(format!("Expected end of input, got {:?}", self.current)));
        }
        Ok(expr)
    }

    // expr → term (('+' | '-') term)*
    fn expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.term()?;

        loop {
            match &self.current {
                Token::Plus => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::add(left, right);
                }
                Token::Minus => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::sub(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // term       → unary (('*' | '/') unary)*
    fn term(&mut self) ->Result<Expr, ParseError> {
        let mut unary = self.unary()?;
        loop {
            match &self.current {
                Token::Star => {
                    self.advance();
                    unary = Expr::mul(unary, self.unary()?);
                }
                Token::Slash => {
                    self.advance();
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
            self.advance();
            let expr = self.unary()?;
            return Ok(Expr::neg(expr));
        }
        self.primary()
    }

    // primary    → NUMBER | '(' expr ')'
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.current {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::number(n))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect_and_advance(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::new(format!("Expected expression, got {:?}", self.current))),
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
            let mut parser = Parser::new(input);
            match parser.parse() {
                Ok(expr) => println!("expr: {:?} = {:?}", expr.to_string(), expr.eval()),
                Err(e)  => println!("error: {:?}", e.message)
            }
        }
    }
}