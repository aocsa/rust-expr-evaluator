use std::fmt;

/// Represents a location in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Error that occurs during lexical analysis.
#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub location: Location,
}

impl LexerError {
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        LexerError {
            message: message.into(),
            location,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {}: {}", self.location, self.message)
    }
}

impl std::error::Error for LexerError {}

/// Error that occurs during parsing.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: Location,
}

impl ParseError {
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        ParseError {
            message: message.into(),
            location,
        }
    }

    pub fn from_lexer_error(err: LexerError) -> Self {
        ParseError {
            message: err.message,
            location: err.location,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}: {}", self.location, self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<LexerError> for ParseError {
    fn from(err: LexerError) -> Self {
        ParseError::from_lexer_error(err)
    }
}

/// Error that occurs during expression evaluation.
#[derive(Debug, Clone)]
pub enum EvalError {
    DivisionByZero,
    Overflow,
    Underflow,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "Division by zero"),
            EvalError::Overflow => write!(f, "Numeric overflow"),
            EvalError::Underflow => write!(f, "Numeric underflow"),
        }
    }
}

impl std::error::Error for EvalError {}
