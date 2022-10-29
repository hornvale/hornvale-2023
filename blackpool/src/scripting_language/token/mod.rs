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
  #[named]
  pub fn synthesize(lexeme: &'source str) -> Self {
    trace_enter!();
    trace_var!(lexeme);
    let r#type = Type::Error;
    trace_var!(r#type);
    let line_number = 0;
    trace_var!(line_number);
    let result = Self {
      r#type,
      lexeme,
      line_number,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
