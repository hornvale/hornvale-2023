use crate::scripting::parser::error::Error as ParserError;
use crate::scripting::scanner::error::Error as ScannerError;

/// Errors encountered during the compilation process.
#[derive(Clone, Debug, Error)]
pub enum Error {
  /// Parser error.
  #[error("an error occurred in the parser ({0})")]
  ParserError(#[from] ParserError),
  /// Scanner error.
  #[error("an error occurred in the scanner ({0})")]
  ScannerError(#[from] ScannerError),
}
