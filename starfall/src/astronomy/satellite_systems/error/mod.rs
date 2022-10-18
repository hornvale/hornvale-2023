use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::satellite_system::error::Error as SatelliteSystemError;

/// Satellite systems errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star.
  HostStarError(HostStarError),
  /// Satellite System.
  SatelliteSystemError(SatelliteSystemError),
  /// No habitable systems found.
  NoHabitableSatelliteSystemsFound,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    HostStarError(host_star_error) => format!(
      "an error occurred in the host star ({})",
      honeyholt_brief!(host_star_error)
    ),
    SatelliteSystemError(satellite_system_error) => format!(
      "an error occurred in the satellite system ({})",
      honeyholt_brief!(satellite_system_error)
    ),
    NoHabitableSatelliteSystemsFound => "no habitable systems could be found".to_string(),
  }
});

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

impl From<SatelliteSystemError> for Error {
  #[named]
  fn from(error: SatelliteSystemError) -> Self {
    Error::SatelliteSystemError(error)
  }
}
