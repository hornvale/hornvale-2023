use super::token::{Token, TokenLiteral, TokenType};
use anyhow::Error as AnyError;
use std::str::FromStr;

/// The `Scanner` type.
#[derive(Clone, Debug, Default, Display, Eq, Hash, PartialEq)]
#[display(fmt = "input: {}, tokens: {:#?}", input, tokens)]
pub struct Scanner<'input> {
  pub tokens: Vec<Token<'input>>,
  pub start: usize,
  pub current: usize,
  pub input: &'input str,
  pub input_bytes: Vec<u8>,
}

impl<'input> Scanner<'input> {
  /// Constructor.
  pub fn new(input: &'input str) -> Self {
    let start = 0;
    let current = 0;
    let input_bytes = input.as_bytes().to_vec();
    let tokens = Vec::new();
    Self {
      start,
      current,
      input,
      input_bytes,
      tokens,
    }
  }

  /// Scan all of the tokens!
  pub fn scan_tokens(&mut self) -> Result<Vec<Token<'input>>, AnyError> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token()?;
    }
    self.tokens.push(Token {
      r#type: TokenType::Eof,
      lexeme: "",
      literal: None,
      entity_id: None,
    });
    Ok(self.tokens.clone())
  }

  /// Scan a single token.
  pub fn scan_token(&mut self) -> Result<(), AnyError> {
    let char = self.advance()?;
    use TokenType::*;
    match char {
      '&' => self.add_token(Ampersand, None),
      '*' => self.add_token(Asterisk, None),
      '@' => self.add_token(At, None),
      '\\' => self.add_token(BackSlash, None),
      '^' => self.add_token(Caret, None),
      ':' => self.add_token(Colon, None),
      ',' => self.add_token(Comma, None),
      '-' => self.add_token(Dash, None),
      '$' => self.add_token(Dollar, None),
      '=' => self.add_token(Equals, None),
      '!' => self.add_token(ExclamationPoint, None),
      '/' => self.add_token(ForwardSlash, None),
      '>' => self.add_token(GreaterThan, None),
      '[' => self.add_token(LeftBrace, None),
      '{' => self.add_token(LeftCurlyBrace, None),
      '(' => self.add_token(LeftParenthesis, None),
      '<' => self.add_token(LessThan, None),
      '%' => self.add_token(Percent, None),
      '.' => self.add_token(Period, None),
      '|' => self.add_token(Pipe, None),
      '+' => self.add_token(Plus, None),
      '#' => self.add_token(Pound, None),
      ']' => self.add_token(RightBrace, None),
      '}' => self.add_token(RightCurlyBrace, None),
      ')' => self.add_token(RightParenthesis, None),
      ';' => self.add_token(Semicolon, None),
      '\'' => self.add_token(SingleQuotation, None),
      '_' => self.add_token(Underscore, None),
      '?' => self.add_token(Question, None),
      '"' => self.scan_string()?,
      char if self.is_digit(char) => self.scan_number()?,
      char if self.is_alpha(char) => self.scan_identifier()?,
      ' ' | '\r' | '\t' | '\n' => {},
      unexpected => bail!("unexpected character '{}'", unexpected),
    };
    Ok(())
  }

  /// Create a token based on a token type.
  pub fn add_token(&mut self, r#type: TokenType, literal: Option<TokenLiteral>) {
    let lexeme = self.get_lexeme();
    let token = Token {
      r#type,
      lexeme,
      literal,
      entity_id: None,
    };
    self.tokens.push(token);
  }

  /// Get the lexeme from the start and current position.
  pub fn get_lexeme(&self) -> &'input str {
    &self.input[self.start..self.current] as _
  }

  /// Advance one character through the input and return it.
  pub fn advance(&mut self) -> Result<char, AnyError> {
    let position = self.current;
    self.current += 1;
    Ok(self.input_bytes[position] as char)
  }

  /// Scan a number.
  pub fn scan_number(&mut self) -> Result<(), AnyError> {
    while self.is_digit(self.peek()) {
      self.advance()?;
    }
    let value = &self.input[self.start..self.current];
    let parsed = value.parse::<i32>().unwrap();
    let number = TokenLiteral::Number(parsed);
    self.add_token(TokenType::Number, Some(number));
    Ok(())
  }

  /// Scan a string.
  pub fn scan_string(&mut self) -> Result<(), AnyError> {
    while self.peek() != '"' && !self.is_at_end() {
      self.advance()?;
    }
    if self.is_at_end() {
      bail!("Unterminated string.");
    }
    self.advance()?;
    let value = &self.input[self.start + 1..self.current - 1];
    self.add_token(TokenType::Literal, Some(TokenLiteral::String(value.to_string())));
    Ok(())
  }

  /// Match an identifier (alphanumeric) (or token).
  pub fn scan_identifier(&mut self) -> Result<(), AnyError> {
    let mut genitive_flag = false;
    while self.is_alpha_numeric(self.peek()) {
      self.advance()?;
    }
    if self.peek() == '\'' {
      self.advance()?;
      genitive_flag = true;
      if self.peek() == 's' {
        self.advance()?;
      }
    }
    let value = &self.input[self.start..self.current];
    let value_type = match TokenType::from_str(value) {
      Ok(token_type) => token_type,
      Err(_) => match genitive_flag {
        true => TokenType::Genitive,
        false => TokenType::Identifier,
      },
    };
    self.add_token(value_type, None);
    Ok(())
  }

  /// Is it a digit?
  pub fn is_digit(&self, char: char) -> bool {
    ('0'..='9').contains(&char)
  }

  /// Is it a letter?
  pub fn is_alpha(&self, char: char) -> bool {
    ('a'..='z').contains(&char) || ('A'..='Z').contains(&char) || char == '_'
  }

  /// Is it alphanumeric?
  pub fn is_alpha_numeric(&self, char: char) -> bool {
    self.is_digit(char) || self.is_alpha(char)
  }

  /// Peek at current character.
  pub fn peek(&self) -> char {
    self.peek_at_offset(0)
  }

  /// Peek at next character.
  pub fn peek_next(&self) -> char {
    self.peek_at_offset(1)
  }

  /// Peek at nth character.
  pub fn peek_at_offset(&self, offset: usize) -> char {
    match self.current + offset >= self.input.len() {
      true => '\0',
      false => self.input_bytes[self.current + offset] as char,
    }
  }

  /// Are we at the end?
  pub fn is_at_end(&self) -> bool {
    self.current >= self.input.len()
  }
}
