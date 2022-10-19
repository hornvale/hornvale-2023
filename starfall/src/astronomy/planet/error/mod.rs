use crate::astronomy::gas_giant_planet::error::Error as GasGiantPlanetError;
use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::terrestrial_planet::error::Error as TerrestrialPlanetError;

/// Planet errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// GasGiantPlanet error.
  #[error("an error occurred in the gas giant planet ({0})")]
  GasGiantPlanetError(#[from] GasGiantPlanetError),
  /// HostStar error.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// TerrestrialPlanet error.
  #[error("an error occurred in the terrestrial planet ({0})")]
  TerrestrialPlanetError(#[from] TerrestrialPlanetError),
  /// The planet type is uninhabitable.
  #[error("the planet type is inherently uninhabitable")]
  UninhabitablePlanetType,
}
