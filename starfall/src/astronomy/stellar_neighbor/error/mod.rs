use crate::astronomy::star_system::error::Error as StarSystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Star System Error.
  #[error("an error occurred in the star ({0})")]
  StarSystemError(#[from] StarSystemError),
}
