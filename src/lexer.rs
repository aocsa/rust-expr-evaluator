pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer {
      chars: input.chars().peekable(),
    }
  }

  fn skip_whitespace(&mut self){ 

  }

  fn read_number(&mut self) -> f64 {
    0.0
  }

}
