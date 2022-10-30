use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Planetary System Error
  #[error("an error occurred in the planetary system ({0})")]
  PlanetarySystemError(#[from] PlanetarySystemError),
}
