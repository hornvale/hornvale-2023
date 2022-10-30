use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::satellite_system::error::Error as SatelliteSystemError;

/// Satellite systems errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Host Star.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// Satellite System.
  #[error("an error occurred in the satellite system ({0})")]
  SatelliteSystemError(#[from] SatelliteSystemError),
  /// No habitable systems found.
  #[error("no habitable satellite systems could be found")]
  NoHabitableSatelliteSystemsFound,
}
