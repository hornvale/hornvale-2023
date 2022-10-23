use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;

pub mod error;
use error::Error;

/// The `Scanner` type.
#[derive(Clone, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[display(fmt = "start: {}, current: {}, line: {}", start, current, line_number)]
pub struct Scanner {
  /// Where this token started.
  pub start: usize,
  /// Where we are currently.
  pub current: usize,
  /// The current line number.
  pub line_number: usize,
  /// The source code.
  pub source: String,
  /// The source bytes.
  pub source_bytes: Vec<u8>,
}

impl Scanner {
  /// Constructor.
  #[named]
  pub fn new(source: &str) -> Self {
    trace_enter!();
    trace_var!(source);
    let start = 0;
    trace_var!(start);
    let current = 0;
    trace_var!(current);
    let line_number = 1;
    trace_var!(line_number);
    let source = source.to_string();
    trace_var!(source);
    let source_bytes = source.as_bytes().to_vec();
    trace_var!(source_bytes);
    let result = Self {
      start,
      current,
      line_number,
      source,
      source_bytes,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Scan a token.
  #[named]
  pub fn scan_token(&mut self) -> Result<Token, Error> {
    trace_enter!();
    self.start = self.current;
    use TokenType::*;
    if self.is_at_end() {
      return Ok(self.make_token(Eof));
    }
    let char = self.advance();
    let result = match char {
      '(' => self.make_token(LeftParenthesis),
      ')' => self.make_token(RightParenthesis),
      '{' => self.make_token(LeftBrace),
      '}' => self.make_token(RightBrace),
      ',' => self.make_token(Comma),
      '.' => self.make_token(Dot),
      '-' => self.make_token(Minus),
      '+' => self.make_token(Plus),
      ';' => self.make_token(Semicolon),
      '*' => self.make_token(Star),
      _ => return Err(Error::UnexpectedCharacter(char)),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
  /*
    #[named]
    pub fn scan_token(&mut self) -> Result<(), ScriptError> {
        '!' => match self.match_current('=') {
          true => self.add_token(BangEqual, None),
          false => self.add_token(Bang, None),
        },
        '=' => match self.match_current('=') {
          true => self.add_token(EqualEqual, None),
          false => self.add_token(Equal, None),
        },
        '>' => match self.match_current('=') {
          true => self.add_token(GreaterThanOrEqual, None),
          false => self.add_token(GreaterThan, None),
        },
        '<' => match self.match_current('=') {
          true => self.add_token(LessThanOrEqual, None),
          false => self.add_token(LessThan, None),
        },
        '/' => match self.peek() {
          '/' => self.match_line_comment(),
          '*' => self.match_multiline_comment(),
          _ => self.add_token(Slash, None),
        },
        ' ' | '\r' | '\t' => {},
        '\n' => self.line_number += 1,
        '"' => self.match_string()?,
        char if self.is_digit(char) => self.match_number(),
        char if self.is_alpha(char) => self.match_identifier(),
        _ => {
          return Err(ScriptError::Error {
            token: None,
            message: format!("Unexpected character: {}", char),
          })
        },
      }
      Ok(())
    }
  */

  /// Advance one character through the source and return it.
  #[named]
  #[inline]
  pub fn advance(&mut self) -> char {
    trace_enter!();
    let position = self.current;
    trace_var!(position);
    self.current += 1;
    let result = self.source_bytes[position] as char;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Are we at the end of the source?
  #[named]
  pub fn is_at_end(&self) -> bool {
    trace_enter!();
    let result = self.current >= self.source.len();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Create a token based on a token type.
  #[named]
  pub fn make_token(&self, r#type: TokenType) -> Token {
    trace_enter!();
    trace_var!(r#type);
    let start = self.start;
    trace_var!(start);
    let length = self.current - self.start;
    trace_var!(length);
    let line_number = self.line_number;
    trace_var!(line_number);
    let result = Token {
      r#type,
      start,
      length,
      line_number,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_scanner() {
    init();
    trace_enter!();
    let scanner = Scanner::default();
    print_var!(scanner);
    trace_exit!();
  }
}
