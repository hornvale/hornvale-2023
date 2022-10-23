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
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    let result = {
      self.host_star.check_habitable()?;
      self.satellite_systems.check_habitable()?;
      Ok(())
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_stellar_mass(&self) -> f64 {
    trace_enter!();
    let result = self.host_star.get_stellar_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    let result = self.host_star.get_stellar_count();
    trace_var!(result);
    trace_exit!();
    result
  }
}
