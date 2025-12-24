use crate::ast::Expr;
use crate::token::Token;

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        // Integer part
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        // Decimal part
        if self.chars.peek() == Some(&'.') {
            num_str.push('.');
            self.chars.next();

            while let Some(&c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    self.chars.next();
                } else {
                    break;
                }
            }
        }
        num_str.parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Token::Eof,
            Some(&c) => match c {
                '0'..='9' => Token::Number(self.read_number()),
                '+' => {
                    self.chars.next();
                    Token::Plus
                }
                '-' => {
                    self.chars.next();
                    Token::Minus
                }
                '*' => {
                    self.chars.next();
                    Token::Star
                }
                '/' => {
                    self.chars.next();
                    Token::Slash
                }
                '(' => {
                    self.chars.next();
                    Token::LeftParen
                }
                ')' => {
                    self.chars.next();
                    Token::RightParen
                }
                _ => panic!("Unexpected character: {}", c),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        // x + 3 * 4
        let mut lexer = Lexer::new("2 + 2 * 2.5");
        loop {
            let token = lexer.next_token();
            println!("token {:?}", token);
            if token == Token::Eof {
                break;
            }
        }
    }
}
