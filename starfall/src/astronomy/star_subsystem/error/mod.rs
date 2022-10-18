use std::convert::From;

use crate::astronomy::distant_binary_star::error::Error as DistantBinaryStarError;
use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Wrap a Distant Binary Star error.
  DistantBinaryStarError(DistantBinaryStarError),
  /// Wrap a Planetary System error.
  PlanetarySystemError(PlanetarySystemError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    DistantBinaryStarError(distant_binary_star_error) => format!(
      "an error occurred in the distant binary star ({})",
      honeyholt_brief!(distant_binary_star_error)
    ),
    PlanetarySystemError(planetary_system_error) => format!(
      "an error occurred in the planetary system ({})",
      honeyholt_brief!(planetary_system_error)
    ),
  }
});

impl From<DistantBinaryStarError> for Error {
  #[named]
  fn from(error: DistantBinaryStarError) -> Self {
    Error::DistantBinaryStarError(error)
  }
}

impl From<PlanetarySystemError> for Error {
  #[named]
  fn from(error: PlanetarySystemError) -> Self {
    Error::PlanetarySystemError(error)
  }
}
