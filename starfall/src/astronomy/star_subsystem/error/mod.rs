use crate::astronomy::distant_binary_star::error::Error as DistantBinaryStarError;
use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Wrap a Distant Binary Star error.
  #[error("an error occurred in the distant binary star ({0})")]
  DistantBinaryStarError(#[from] DistantBinaryStarError),
  /// Wrap a Planetary System error.
  #[error("an error occurred in the planetary system ({0})")]
  PlanetarySystemError(#[from] PlanetarySystemError),
}
