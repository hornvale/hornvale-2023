use crate::astronomy::stellar_neighbor::error::Error as StellarNeighborError;

/// Stellar Neighborhood errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Stellar Neighbor Error.
  StellarNeighborError(StellarNeighborError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    StellarNeighborError(stellar_neighbor_error) => format!(
      "an error occurred in the stellar neighbor ({})",
      honeyholt_brief!(stellar_neighbor_error)
    ),
  }
});

impl From<StellarNeighborError> for Error {
  #[named]
  fn from(error: StellarNeighborError) -> Self {
    Error::StellarNeighborError(error)
  }
}
