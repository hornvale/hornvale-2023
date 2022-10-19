use crate::astronomy::close_binary_star::error::Error as CloseBinaryStarError;
use crate::astronomy::star::error::Error as StarError;

/// Moons errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Close Binary Star Error
  #[error("an error occurred in the close binary star ({0})")]
  CloseBinaryStarError(#[from] CloseBinaryStarError),
  /// Star Error
  #[error("an error occurred in the star ({0})")]
  StarError(#[from] StarError),
}
