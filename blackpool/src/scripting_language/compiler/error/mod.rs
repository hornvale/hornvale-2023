use crate::scripting_language::parser::error::Error as ParserError;
use crate::scripting_language::scanner::error::Error as ScannerError;

/// Errors encountered during the compilation process.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
  /// Parser error.
  #[error("an error occurred in the parser ({0})")]
  ParserError(#[from] ParserError),
  /// Scanner error.
  #[error("an error occurred in the scanner ({0})")]
  ScannerError(#[from] ScannerError),
}
