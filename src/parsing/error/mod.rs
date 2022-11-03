use std::error::Error as StdError;
use std::io::Error as IoError;

use crate::commands::error::Error as CommandError;
use crate::direction::error::Error as DirectionError;

/// Errors encountered in parsing.
#[derive(Debug, Error)]
pub enum Error {
  /// A standard error occurred.
  #[error("an error occurred ({0})")]
  StandardError(#[from] Box<dyn StdError>),
  /// An I/O error occurred.
  #[error("an error occurred ({0})")]
  IoError(#[from] IoError),
  /// A command error occurred.
  #[error("an error occurred ({0})")]
  CommandError(#[from] CommandError),
  /// A direction error occurred.
  #[error("a direction error occurred ({0})")]
  DirectionError(#[from] DirectionError),
}
