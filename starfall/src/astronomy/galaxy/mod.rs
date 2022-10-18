use crate::astronomy::stellar_neighborhood::StellarNeighborhood;

pub mod constants;
pub mod constraints;
pub mod error;

/// A `Galaxy` is the "outermost" or largest-scale object.
///
/// It's a wrapper around `StellarNeighborhood`.
#[derive(Clone, Debug, PartialEq)]
pub struct Galaxy {
  /// This might be plural someday.  For now, we don't care.
  pub stellar_neighborhood: StellarNeighborhood,
}
