use crate::astronomy::star_system::error::Error as StarSystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star System Error.
  StarSystemError(StarSystemError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    StarSystemError(star_system_error) => format!(
      "an error occurred while generating the star system ({})",
      honeyholt_brief!(star_system_error)
    ),
  }
});

impl From<StarSystemError> for Error {
  #[named]
  fn from(error: StarSystemError) -> Self {
    Error::StarSystemError(error)
  }
}
