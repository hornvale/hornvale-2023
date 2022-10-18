use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Planetary System Error
  PlanetarySystemError(PlanetarySystemError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    PlanetarySystemError(planetary_system_error) => format!(
      "an error occurred in the planetary system ({})",
      honeyholt_brief!(planetary_system_error)
    ),
  }
});

impl From<PlanetarySystemError> for Error {
  #[named]
  fn from(error: PlanetarySystemError) -> Self {
    Error::PlanetarySystemError(error)
  }
}
