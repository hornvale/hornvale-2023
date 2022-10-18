use crate::astronomy::moons::Moons;
use crate::astronomy::planet::Planet;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// A `SatelliteSystem` is a collection of a `Planet` and `Moons`.
#[derive(Clone, Debug, PartialEq)]
pub struct SatelliteSystem {
  /// The planet.
  pub planet: Planet,
  /// The moons.
  pub moons: Moons,
}

impl SatelliteSystem {
  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    let result = {
      self.planet.check_habitable()?;
      // Perhaps someday.
      // self.moons.check_habitable()?;
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
