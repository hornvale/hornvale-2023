use crate::astronomy::host_star::error::Error as HostStarError;

/// GasGiantPlanet errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Host Star Error.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
}
