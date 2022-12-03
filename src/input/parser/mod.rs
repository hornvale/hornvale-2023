use super::token::{Token, TokenType};
use super::ParserData;
use crate::command::*;
use anyhow::Error as AnyError;

/// The `Parser` type.
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
#[display(fmt = "input: {}, tokens: {:#?}", input, tokens)]
pub struct Parser<'input> {
  pub input: &'input str,
  pub tokens: Vec<Token<'input>>,
  pub current: usize,
}

impl<'input> Parser<'input> {
  /// Constructor.
  pub fn new(tokens: Vec<Token<'input>>, input: &'input str) -> Self {
    let current = 0;
    Self { input, tokens, current }
  }

  /// Parse input.
  pub fn parse(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.parse_command(data)
  }

  /// Parse a command.
  pub fn parse_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    if self.check(TokenType::Direction)? && self.peek_next()?.r#type == TokenType::Eof {
      return self.parse_direction_command(data);
    }
    if self.match_verb()? {
      self.match_noun_phrase(data)?;
      return self.parse_action_command(data);
    }
    self.parse_order_command(data)
  }

  /// Parse an order command; that is, tell another actor to do something.
  pub fn parse_order_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    if self.match_noun_phrase(data)? {
      self.consume(
        TokenType::Comma,
        &format!("Expected a comma after the addressee, {}", self.previous()?),
      )?;
    }
    Command::from_data(original_input, string, self.tokens.clone(), data)
  }

  /// Parse an action command; that is, a simple command.
  pub fn parse_action_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.advance()?;
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    Command::from_data(original_input, string, self.tokens.clone(), data)
  }

  /// Parse a direction command, e.g. "sw".
  pub fn parse_direction_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.consume(TokenType::Direction, "Expected a valid direction.")?;
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    let mut tokens = vec![Token {
      r#type: TokenType::Go,
      lexeme: "go",
      literal: None,
      entity_id: None,
    }];
    tokens.append(&mut self.tokens);
    Command::from_data(original_input, string, tokens, data)
  }

  /// Look for a match to a noun phrase.
  pub fn match_noun_phrase(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    while self.match_descriptor(data)? {
      self.current += 1;
    }
    if self.match_noun(data)? {
      self.current += 1;
    } else {
      return Ok(false);
    }
    Ok(true)
  }

  /// Look for a match to a descriptor.
  pub fn match_descriptor(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    if self.check(TokenType::Article)? {
      return Ok(true);
    }
    if self.check(TokenType::Genitive)? {
      return Ok(true);
    }
    let adjectives = data.get_adjectives()?;
    let lexeme = self.peek_next()?.lexeme;
    let next = self.current + 1;
    if adjectives.iter().any(|adjective| adjective == lexeme) {
      self.tokens[next].r#type = TokenType::Adjective;
      return Ok(true);
    }
    Ok(false)
  }

  /// Look for a match to a noun.
  pub fn match_noun(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    let nouns = data.get_nouns()?;
    let lexeme = self.peek_next()?.lexeme;
    let next = self.current + 1;
    for (noun, entity_id) in nouns {
      if noun == lexeme {
        self.tokens[next].r#type = TokenType::Noun;
        self.tokens[next].entity_id = Some(entity_id);
        return Ok(true);
      }
    }
    Ok(false)
  }

  /// Look for a match to a verb.
  pub fn match_verb(&mut self) -> Result<bool, AnyError> {
    Ok(self.peek()?.is_verb())
  }

  /// Look for a match of the current token.
  pub fn r#match(&mut self, types: Vec<TokenType>) -> Result<bool, AnyError> {
    for r#type in types {
      if self.check(r#type)? {
        self.advance()?;
        return Ok(true);
      }
    }
    Ok(false)
  }

  /// Get the raw arguments to the command (for Echo, Eval, etc).
  pub fn get_raw_arguments(&self) -> Result<String, AnyError> {
    let result = self.tokens[1..]
      .iter()
      .map(|token| token.lexeme.to_string())
      .collect::<Vec<String>>()
      .join(" ");
    Ok(result)
  }

  /// Check that the next token is a specific type.
  pub fn check(&mut self, r#type: TokenType) -> Result<bool, AnyError> {
    if self.is_at_end()? {
      return Ok(false);
    }
    Ok(self.peek()?.r#type == r#type)
  }

  /// Advance to the next token.
  pub fn advance(&mut self) -> Result<Token, AnyError> {
    if !self.is_at_end()? {
      self.current += 1;
    }
    self.previous()
  }

  /// Have we processed all of the tokens?
  pub fn is_at_end(&self) -> Result<bool, AnyError> {
    Ok(self.peek()?.r#type == TokenType::Eof)
  }

  /// Peek at the current token.
  pub fn peek(&self) -> Result<Token, AnyError> {
    self.peek_at_offset(0)
  }

  /// Peek at the next token.
  pub fn peek_next(&self) -> Result<Token, AnyError> {
    self.peek_at_offset(1)
  }

  /// Peek at nth token.
  pub fn peek_at_offset(&self, offset: usize) -> Result<Token, AnyError> {
    match self.current + offset >= self.tokens.len() {
      true => Ok(Token {
        r#type: TokenType::Eof,
        lexeme: "",
        literal: None,
        entity_id: None,
      }),
      false => Ok(self.tokens[self.current + offset].clone()),
    }
  }

  /// Show the previous token.
  pub fn previous(&self) -> Result<Token, AnyError> {
    Ok(self.tokens.get(self.current - 1).unwrap().clone())
  }

  /// Consume the next token, or throw an error if it is not matched.
  pub fn consume<'error>(&mut self, r#type: TokenType, message: &'error str) -> Result<Token, AnyError> {
    if self.check(r#type)? {
      return self.advance();
    }
    bail!("{}", message);
  }

}
