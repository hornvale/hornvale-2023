use crate::astronomy::stellar_neighbor::error::Error as StellarNeighborError;

/// Stellar Neighborhood errors.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Stellar Neighbor Error.
  #[error("an error occurred in the stellar neighbor ({0})")]
  StellarNeighborError(#[from] StellarNeighborError),
}
