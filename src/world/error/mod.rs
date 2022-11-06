use anyhow::Error as AnyError;

/// Errors encountered in the world.
///
/// This describes the error type that is returned from an attempt to query or
/// affect the World.
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
}
