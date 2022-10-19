use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Planetary System Error
  #[error("an error occurred in the planetary system ({0})")]
  PlanetarySystemError(#[from] PlanetarySystemError),
}
