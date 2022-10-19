use crate::astronomy::host_star::error::Error as HostStarError;

/// GasGiantPlanet errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Host Star Error.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
}
