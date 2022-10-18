use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::satellite_systems::error::Error as SatelliteSystemsError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star
  HostStarError(HostStarError),
  /// Satellite Systems
  SatelliteSystemsError(SatelliteSystemsError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    HostStarError(host_star_error) => format!(
      "an error occurred in the host star ({})",
      honeyholt_brief!(host_star_error)
    ),
    SatelliteSystemsError(satellite_systems_error) => format!(
      "an error occurred in the satellite systems ({})",
      honeyholt_brief!(satellite_systems_error)
    ),
  }
});

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

impl From<SatelliteSystemsError> for Error {
  #[named]
  fn from(error: SatelliteSystemsError) -> Self {
    Error::SatelliteSystemsError(error)
  }
}
