pub mod error;
pub mod r#type;
use r#type::Type;

/// The `Token` type.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Error, Hash, PartialEq, Serialize)]
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
