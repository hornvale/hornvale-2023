use anyhow::Error as AnyError;

use crate::world::error::Error as WorldError;

/// Errors encountered in actions.
///
/// This describes the error type that is returned from an attempt to execute
/// an action.
///
/// Ideally, this type should probably be an enum, so that callers can use the
/// match constructs to respond appropriately to different underlying causes.
#[derive(Debug, Error)]
pub enum Error {
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
  /// Any error occurred.
  #[error(transparent)]
  AnyError(#[from] AnyError),
  /// A world error occurred.
  #[error("a world error occurred ({0})")]
  WorldError(#[from] WorldError),
}
