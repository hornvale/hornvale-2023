use crate::astronomy::satellite_system::SatelliteSystem;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// The `SatelliteSystems` object wraps a vector of `SatelliteSystem` objects.
#[derive(Clone, Debug, PartialEq)]
pub struct SatelliteSystems {
  /// SatelliteSystem objects.
  pub satellite_systems: Vec<SatelliteSystem>,
}

impl SatelliteSystems {
  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
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
}
