use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::satellite_systems::error::Error as SatelliteSystemsError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Host Star
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// Satellite Systems
  #[error("an error occurred in the satellite systems ({0})")]
  SatelliteSystemsError(#[from] SatelliteSystemsError),
}
