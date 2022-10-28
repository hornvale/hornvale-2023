use std::str::FromStr;

use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;

pub mod error;
use error::Error;

/// The `Scanner` type.
#[derive(Clone, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[display(fmt = "start: {}, current: {}, line: {}", start, current, line_number)]
pub struct Scanner<'source> {
  /// Where this token started.
  pub start: usize,
  /// Where we are currently.
  pub current: usize,
  /// The current line number.
  pub line_number: usize,
  /// The source code.
  pub source: &'source str,
  /// The source bytes.
  pub source_bytes: Vec<u8>,
}

impl<'source> Scanner<'source> {
  /// Constructor.
  #[named]
  pub fn new(source: &'source str) -> Self {
    trace_enter!();
    trace_var!(source);
    let start = 0;
    trace_var!(start);
    let current = 0;
    trace_var!(current);
    let line_number = 1;
    trace_var!(line_number);
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
    self.skip_whitespace();
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
      '/' => self.make_token(Slash),
      '!' => match self.match_current('=') {
        true => self.make_token(BangEqual),
        false => self.make_token(Bang),
      },
      '=' => match self.match_current('=') {
        true => self.make_token(EqualEqual),
        false => self.make_token(Equal),
      },
      '>' => match self.match_current('=') {
        true => self.make_token(GreaterThanOrEqual),
        false => self.make_token(GreaterThan),
      },
      '<' => match self.match_current('=') {
        true => self.make_token(LessThanOrEqual),
        false => self.make_token(LessThan),
      },
      '"' => self.match_string()?,
      char if self.is_digit(char) => self.match_number()?,
      char if self.is_alpha(char) => self.match_identifier()?,
      _ => return Err(Error::UnexpectedCharacter(char)),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

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

  /// Does the current character match the one specified?
  #[named]
  pub fn match_current(&mut self, char: char) -> bool {
    trace_enter!();
    trace_var!(char);
    if self.is_at_end() {
      return false;
    }
    if self.source_bytes[self.current] as char != char {
      return false;
    }
    self.current += 1;
    trace_exit!();
    true
  }

  /// Try to match and create a token out of a number.
  #[named]
  pub fn match_number(&mut self) -> Result<Token, Error> {
    trace_enter!();
    while self.is_digit(self.peek()) {
      self.advance();
    }
    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      self.advance();
      while self.is_digit(self.peek()) {
        self.advance();
      }
    }
    let value = &self.source[self.start..self.current];
    trace_var!(value);
    let result = self.make_token(TokenType::Number);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Try to match and create a token out of a string.
  #[named]
  pub fn match_string(&mut self) -> Result<Token, Error> {
    trace_enter!();
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line_number += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      self.inject_error_token("Unterminated string.");
      return Err(Error::UnterminatedString(self.line_number));
    }
    self.advance();
    let result = self.make_token(TokenType::String);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  #[named]
  pub fn match_identifier(&mut self) -> Result<Token, Error> {
    trace_enter!();
    while self.is_alpha_numeric(self.peek()) {
      self.advance();
    }
    let value = &self.source[self.start..self.current];
    trace_var!(value);
    let value_type = match TokenType::from_str(value) {
      Ok(token_type) => token_type,
      Err(_) => TokenType::Identifier,
    };
    trace_var!(value_type);
    let result = self.make_token(value_type);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  #[named]
  pub fn is_digit(&self, char: char) -> bool {
    trace_enter!();
    trace_var!(char);
    let result = ('0'..='9').contains(&char);
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  pub fn is_alpha(&self, char: char) -> bool {
    trace_enter!();
    trace_var!(char);
    let result = ('a'..='z').contains(&char) || ('A'..='Z').contains(&char) || char == '_';
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  pub fn is_alpha_numeric(&self, char: char) -> bool {
    trace_enter!();
    trace_var!(char);
    let result = self.is_digit(char) || self.is_alpha(char);
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  pub fn inject_error_token(&self, message: &'static str) -> Token {
    trace_enter!();
    trace_var!(message);
    let mut result = Token::synthesize(TokenType::ScannerError(message));
    result.line_number = self.line_number;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Match a single-line comment.
  #[named]
  pub fn match_line_comment(&mut self) {
    trace_enter!();
    while self.peek() != '\n' && !self.is_at_end() {
      self.advance();
    }
    trace_exit!();
  }

  /// Match a multi-line comment.
  #[named]
  pub fn match_multiline_comment(&mut self) {
    trace_enter!();
    while !(self.is_at_end() || self.peek_next() == '*' && self.peek_at_offset(2) == '/') {
      self.advance();
    }
    // Last trailing non-asterisk, non-forward-slash character.
    self.advance();
    // Trailing asterisk.
    self.advance();
    // Trailing forward slash.
    self.advance();
    trace_exit!();
  }

  /// Peek at the current character, but don't advance.
  #[named]
  pub fn peek(&self) -> char {
    trace_enter!();
    let result = self.peek_at_offset(0);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Peek at the next character.
  #[named]
  pub fn peek_next(&self) -> char {
    trace_enter!();
    let result = self.peek_at_offset(1);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Peek at a character at a specified offset.
  #[named]
  pub fn peek_at_offset(&self, offset: usize) -> char {
    trace_enter!();
    let result = match self.current + offset >= self.source.len() {
      true => '\0',
      false => self.source_bytes[self.current + offset] as char,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Skip all the whitespace!
  #[named]
  pub fn skip_whitespace(&mut self) {
    trace_enter!();
    loop {
      match self.peek() {
        '\n' => {
          self.line_number += 1;
          self.advance();
        },
        ' ' | '\r' | '\t' => {
          self.advance();
        },
        '/' => match self.peek_next() {
          '/' => self.match_line_comment(),
          '*' => self.match_multiline_comment(),
          _ => break,
        },
        _ => break,
      }
    }
    trace_exit!();
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
    test_scanner_tokens!(
      "".into(),
      [Ok(Token {
        r#type: TokenType::Eof,
        start: 0,
        length: 0,
        line_number: 1,
      })]
    );
    use TokenType::*;
    let test_cases = vec![
      ("(", LeftParenthesis),
      (")", RightParenthesis),
      ("{", LeftBrace),
      ("}", RightBrace),
      (",", Comma),
      (".", Dot),
      ("-", Minus),
      ("+", Plus),
      (";", Semicolon),
      ("*", Star),
      ("/", Slash),
      ("/a", Slash),
      ("!s", Bang),
      ("<q", LessThan),
      (">q", GreaterThan),
    ];
    for (string, r#type) in test_cases.iter() {
      test_scanner_tokens!(
        string,
        [Ok(Token {
          r#type: *r#type,
          start: 0,
          length: 1,
          line_number: 1,
        })]
      );
    }
    let test_cases2 = vec![
      ("!=", BangEqual),
      ("==", EqualEqual),
      (">=", GreaterThanOrEqual),
      ("<=", LessThanOrEqual),
    ];
    for (string, r#type) in test_cases2.iter() {
      test_scanner_tokens!(
        string,
        [Ok(Token {
          r#type: *r#type,
          start: 0,
          length: 2,
          line_number: 1,
        })]
      );
    }
    test_scanner_tokens!(
      "/a",
      [
        Ok(Token {
          r#type: Slash,
          start: 0,
          length: 1,
          line_number: 1,
        }),
        Ok(Token {
          r#type: Identifier,
          start: 1,
          length: 1,
          line_number: 1,
        })
      ]
    );
    test_scanner_tokens!(
      "/* TEST */",
      [Ok(Token {
        r#type: Eof,
        start: 10,
        length: 0,
        line_number: 1,
      })]
    );
    test_scanner_tokens!(
      "1312.1231215123",
      [Ok(Token {
        r#type: Number,
        start: 0,
        length: 15,
        line_number: 1,
      })]
    );
    test_scanner_tokens!(
      "\"goat\"",
      [Ok(Token {
        r#type: String,
        start: 0,
        length: 6,
        line_number: 1,
      })]
    );
    test_scanner_tokens!(
      "// single-line comment",
      [Ok(Token {
        r#type: Eof,
        start: 22,
        length: 0,
        line_number: 1,
      })]
    );
    test_scanner_tokens!(
      "\n",
      [Ok(Token {
        r#type: Eof,
        start: 1,
        length: 0,
        line_number: 2,
      })]
    );
    trace_exit!();
  }
}
