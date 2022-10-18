use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_subsystem::error::Error as StarSubsystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// StarSubsystem Error.
  StarSubsystemError(StarSubsystemError),
  /// No suitable StarSubsystems found.
  NoSuitableSubsystemsCouldBeGenerated,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    NoSuitableSubsystemsCouldBeGenerated => "no suitable subsystems could be generated".to_string(),
    StarSubsystemError(star_subsystem_error) => format!(
      "an error occurred in the star subsystem ({})",
      honeyholt_brief!(star_subsystem_error)
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

impl From<StarSubsystemError> for Error {
  #[named]
  fn from(error: StarSubsystemError) -> Self {
    Error::StarSubsystemError(error)
  }
}
