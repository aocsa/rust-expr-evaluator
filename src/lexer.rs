use crate::error::{LexerError, Location};
use crate::token::Token;

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    /// Returns the current location in the source.
    pub fn location(&self) -> Location {
        Location::new(self.line, self.column)
    }

    fn advance_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, LexerError> {
        let start_location = self.location();
        let mut num_str = String::new();

        // Integer part
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance_char();
            } else {
                break;
            }
        }

        // Decimal part
        if self.chars.peek() == Some(&'.') {
            num_str.push('.');
            self.advance_char();

            let mut has_decimal_digits = false;
            while let Some(&c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    self.advance_char();
                    has_decimal_digits = true;
                } else {
                    break;
                }
            }

            if !has_decimal_digits {
                return Err(LexerError::new(
                    "Expected digits after decimal point",
                    start_location,
                ));
            }
        }

        num_str
            .parse()
            .map_err(|_| LexerError::new(format!("Invalid number: {}", num_str), start_location))
    }

    pub fn next_token(&mut self) -> Result<(Token, Location), LexerError> {
        self.skip_whitespace();

        let location = self.location();

        match self.chars.peek() {
            None => Ok((Token::Eof, location)),
            Some(&c) => match c {
                '0'..='9' => Ok((Token::Number(self.read_number()?), location)),
                '+' => {
                    self.advance_char();
                    Ok((Token::Plus, location))
                }
                '-' => {
                    self.advance_char();
                    Ok((Token::Minus, location))
                }
                '*' => {
                    self.advance_char();
                    Ok((Token::Star, location))
                }
                '/' => {
                    self.advance_char();
                    Ok((Token::Slash, location))
                }
                '(' => {
                    self.advance_char();
                    Ok((Token::LeftParen, location))
                }
                ')' => {
                    self.advance_char();
                    Ok((Token::RightParen, location))
                }
                _ => Err(LexerError::new(
                    format!("Unexpected character: '{}'", c),
                    location,
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("2 + 2 * 2.5");
        loop {
            let (token, location) = lexer.next_token().expect("Failed to tokenize");
            println!("token {:?} at {:?}", token, location);
            if token == Token::Eof {
                break;
            }
        }
    }

    #[test]
    fn test_lexer_error() {
        let mut lexer = Lexer::new("2 + @");
        assert!(lexer.next_token().is_ok()); // 2
        assert!(lexer.next_token().is_ok()); // +
        let err = lexer.next_token();
        assert!(err.is_err());
        let err = err.unwrap_err();
        assert_eq!(err.location.line, 1);
        assert_eq!(err.location.column, 5);
    }

    #[test]
    fn test_lexer_multiline() {
        let mut lexer = Lexer::new("1 +\n2");
        assert!(lexer.next_token().is_ok()); // 1
        assert!(lexer.next_token().is_ok()); // +
        let (token, location) = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number(2.0));
        assert_eq!(location.line, 2);
    }
}
