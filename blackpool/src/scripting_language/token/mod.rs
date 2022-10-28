pub mod error;
pub mod r#type;
use r#type::Type;

/// The `Token` type.
#[derive(Clone, Copy, Debug, Display, Eq, Error, Hash, PartialEq)]
#[display(
  fmt = "type: {}, start: {}, length: {}, line: {}",
  r#type,
  start,
  length,
  line_number
)]
pub struct Token {
  /// The type of this token.
  pub r#type: Type,
  /// The start index.
  pub start: usize,
  /// The length of the token.
  pub length: usize,
  /// The line number.
  pub line_number: usize,
}

impl Token {
  /// Constructor.
  #[named]
  pub fn synthesize(r#type: Type) -> Self {
    trace_enter!();
    trace_var!(r#type);
    let start = 0;
    trace_var!(start);
    let length = 0;
    trace_var!(length);
    let line_number = 0;
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
