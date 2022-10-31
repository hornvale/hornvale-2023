use std::error::Error as StdError;
use std::io::Error as IoError;

/// Errors encountered in interpreting.
#[derive(Debug, Error)]
pub enum Error {
  /// A standard error occurred.
  #[error("an error occurred ({0})")]
  StandardError(#[from] Box<dyn StdError>),
  /// An I/O error occurred.
  #[error("an error occurred ({0})")]
  IoError(#[from] IoError),
}
