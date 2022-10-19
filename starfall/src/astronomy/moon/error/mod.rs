use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::planet::error::Error as PlanetError;

/// Moon errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Host Star Error.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// Planet Error.
  #[error("an error occurred in the planet ({0})")]
  PlanetError(#[from] PlanetError),
}
