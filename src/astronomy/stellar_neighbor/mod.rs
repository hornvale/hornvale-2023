use crate::astronomy::star_system::StarSystem;

pub mod constraints;
pub mod error;
pub mod math;

/// The `StellarNeighbor` class.
///
/// No, not someone who brings you brownies when you move into the area.
///
/// This is just a combination of a fully-fledged star system and a set of 3-D
/// coordinates so that we can place it relative to our primary star system.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StellarNeighbor {
  /// Each coordinate (x,y,z) is a distance (in light years) from the origin.
  pub coordinates: (f64, f64, f64),
  /// The details of this particular star system.
  pub star_system: StarSystem,
  /// The distance from the origin.
  pub distance: f64,
  /// The name of the primary star.
  pub name: String,
}

impl StellarNeighbor {
  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.

  pub fn get_stellar_mass(&self) -> f64 {
    self.star_system.get_stellar_mass()
  }

  /// Retrieve or calculate the total number of stars in the system.

  pub fn get_stellar_count(&self) -> u8 {
    self.star_system.get_stellar_count()
  }
}
