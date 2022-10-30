use crate::astronomy::host_star::error::Error as HostStarError;

/// TerrestrialPlanet errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Host Star.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// Pluto, also Minnesota.
  #[error("not habitable because it is too cold")]
  TooColdToSupportConventionalLife,
  /// Hell, or Las Vegas.
  #[error("not habitable because it is too hot")]
  TooHotToSupportConventionalLife,
  /// Hard to fight when people keep floating off into space.
  #[error("not habitable because its gravity is too low")]
  GravityTooLowToSupportConventionalLife,
  /// Just sounds kinda lame.
  #[error("not habitable because its gravity is too high")]
  GravityTooHighToSupportConventionalLife,
  /// Oxygen unstable in this atmosphere.
  #[error("not habitable because it cannot retain oxygen")]
  AtmosphereUnstableForOxygen,
  /// Carbon Dioxide unstable in this atmosphere.
  #[error("not habitable because it cannot retain carbon dioxide")]
  AtmosphereUnstableForCarbonDioxide,
  /// Argon unstable in this atmosphere.
  #[error("not habitable because it cannot retain argon")]
  AtmosphereUnstableForArgon,
  /// Nitrogen unstable in this atmosphere.
  #[error("not habitable because it cannot retain nitrogen")]
  AtmosphereUnstableForNitrogen,
}
