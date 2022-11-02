use crate::astronomy::host_star::HostStar;
use crate::astronomy::satellite_systems::SatelliteSystems;

pub mod constraints;
pub mod error;
use error::Error;

/// A `PlanetarySystem` is a `HostStar` and 0+ `SatelliteSystem` objects.
///
/// So a `PlanetarySystem` does not necessarily include planets.  This is
/// confusing and I don't really like it, but I don't have a better name
/// for it.  Yet.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlanetarySystem {
  pub host_star: HostStar,
  pub satellite_systems: SatelliteSystems,
}

impl PlanetarySystem {
  /// Indicate whether this star is capable of supporting conventional life.

  pub fn check_habitable(&self) -> Result<(), Error> {
    {
      self.host_star.check_habitable()?;
      self.satellite_systems.check_habitable()?;
      Ok(())
    }
  }

  /// Indicate whether this star is capable of supporting conventional life.

  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.

  pub fn get_stellar_mass(&self) -> f64 {
    self.host_star.get_stellar_mass()
  }

  /// Retrieve or calculate the total number of stars in the system.

  pub fn get_stellar_count(&self) -> u8 {
    self.host_star.get_stellar_count()
  }
}
