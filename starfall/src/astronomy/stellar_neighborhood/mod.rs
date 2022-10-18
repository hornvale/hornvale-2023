use crate::astronomy::stellar_neighbor::*;

pub mod constants;
pub mod constraints;
pub mod error;

/// The `StellarNeighborhood` type.
///
/// This is mostly a container for star systems.
///
/// We carve out a spherical section, a few light years or so in radius, and
/// generate some companion star systems.  These are likely to be other class V
/// stars, possibly with planets of their own.
///
/// Why?  Well, just to add a little color to the night sky.
#[derive(Clone, Debug, PartialEq)]
pub struct StellarNeighborhood {
  /// The radius of this neighborhood, measured in light years.
  pub radius: f64,
  /// The stellar density of this neighborhood, measured in stars per cubic
  /// light year.  This is not terribly useful once the neighborhood has
  /// been generated, but we keep it around for posterity.
  pub density: f64,
  /// Stellar "neighbors", which is a glorified tuple of three-dimensional
  /// coordinates and a star system.
  pub neighbors: Vec<StellarNeighbor>,
  /// The number of stars in this stellar neighborhood.
  pub star_count: usize,
}

impl StellarNeighborhood {}
