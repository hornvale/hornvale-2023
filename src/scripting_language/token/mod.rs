pub mod error;
pub mod r#type;
use r#type::Type;

/// The `Token` type.
#[derive(Clone, Copy, Debug, Display, Eq, Error, Hash, PartialEq)]
#[display(fmt = "type: {}, lexeme: {}", r#type, lexeme)]
pub struct Token<'source> {
  /// The type of this token.
  pub r#type: Type,
  /// The lexeme.
  pub lexeme: &'source str,
  /// The line number.
  pub line_number: usize,
}

impl<'source> Token<'source> {
  /// Constructor.

  pub fn synthesize(lexeme: &'source str) -> Self {
    let r#type = Type::Error;

    let line_number = 0;

    Self {
      r#type,
      lexeme,
      line_number,
    }
  }
}
