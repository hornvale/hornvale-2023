use crate::scripting_language::scanner::error::Error as ScannerError;
use crate::scripting_language::token::r#type::Type as TokenType;

/// Errors encountered during the parsing process.
#[derive(Clone, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Unknown error.
  #[error("an unknown error occurred")]
  UnknownError,
  /// Unexpected token.
  #[error("unexpected token {0} ({1})")]
  UnexpectedTokenError(TokenType, String),
  /// Multiple errors occurred.
  #[error("multiple errors occurred ({0:#?})")]
  MultipleErrors(Vec<String>),
  /// Scanner error.
  #[error("an error occurred in the scanner ({0})")]
  ScannerError(#[from] ScannerError),
}
