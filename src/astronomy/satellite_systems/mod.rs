use crate::astronomy::satellite_system::SatelliteSystem;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// The `SatelliteSystems` object wraps a vector of `SatelliteSystem` objects.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SatelliteSystems {
  /// SatelliteSystem objects.
  pub satellite_systems: Vec<SatelliteSystem>,
}

impl SatelliteSystems {
  /// Indicate whether this star is capable of supporting conventional life.

  pub fn check_habitable(&self) -> Result<(), Error> {
    let result = {
      let any = self
        .satellite_systems
        .iter()
        .any(|satellite_system| satellite_system.is_habitable());
      if !any {
        return Err(Error::NoHabitableSatelliteSystemsFound);
      }
      Ok(())
    };

    result
  }

  /// Indicate whether this star is capable of supporting conventional life.

  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }
}
