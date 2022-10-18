use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::moon::error::Error as MoonError;
use crate::astronomy::moons::error::Error as MoonsError;
use crate::astronomy::planet::error::Error as PlanetError;

/// Moon-related errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star Error.
  HostStarError(HostStarError),
  /// Moon Error.
  MoonError(MoonError),
  /// Moons Error.
  MoonsError(MoonsError),
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
    MoonError(moon_error) => format!("an error occurred in the moon ({})", honeyholt_brief!(moon_error)),
    MoonsError(moons_error) => format!("an error occurred in the moons ({})", honeyholt_brief!(moons_error)),
    PlanetError(planet_error) => format!("an error occurred in the planet ({})", honeyholt_brief!(planet_error)),
  }
});

impl From<MoonError> for Error {
  #[named]
  fn from(error: MoonError) -> Self {
    Error::MoonError(error)
  }
}

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

impl From<MoonsError> for Error {
  #[named]
  fn from(error: MoonsError) -> Self {
    Error::MoonsError(error)
  }
}

impl From<PlanetError> for Error {
  #[named]
  fn from(error: PlanetError) -> Self {
    Error::PlanetError(error)
  }
}
