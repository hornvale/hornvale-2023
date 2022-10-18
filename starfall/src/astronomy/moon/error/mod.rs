use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::planet::error::Error as PlanetError;

/// Moon errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star Error.
  HostStarError(HostStarError),
  /// Planet Error.
  PlanetError(PlanetError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    HostStarError(host_star_error) => format!(
      "an error occurred in the host star ({})",
      honeyholt_brief!(host_star_error)
    ),
    PlanetError(planet_error) => format!("an error occurred in the planet ({})", honeyholt_brief!(planet_error)),
  }
});

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

impl From<PlanetError> for Error {
  #[named]
  fn from(error: PlanetError) -> Self {
    Error::PlanetError(error)
  }
}
