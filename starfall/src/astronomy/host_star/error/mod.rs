use crate::astronomy::close_binary_star::error::Error as CloseBinaryStarError;
use crate::astronomy::star::error::Error as StarError;

/// Moons errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Close Binary Star Error
  CloseBinaryStarError(CloseBinaryStarError),
  /// Star Error
  StarError(StarError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    CloseBinaryStarError(close_binary_star_error) => format!(
      "an error occurred in the close binary star ({})",
      honeyholt_brief!(close_binary_star_error)
    ),
    StarError(star_error) => format!(
      "an error occurred while generating the star ({})",
      honeyholt_brief!(star_error)
    ),
  }
});

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}

impl From<CloseBinaryStarError> for Error {
  #[named]
  fn from(error: CloseBinaryStarError) -> Self {
    Error::CloseBinaryStarError(error)
  }
}
