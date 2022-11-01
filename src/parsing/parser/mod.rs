use crate::parsing::error::Error;

/// The `Parser` trait.
pub trait Parser {
  /// Parse two (or more) words of input.
  fn parse_input(&mut self, input: &str) -> Result<Option<String>, Error>;
}
