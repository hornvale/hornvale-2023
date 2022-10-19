use rand::prelude::*;
use std::default::Default;

use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
use crate::astronomy::planetary_system::error::Error;
use crate::astronomy::planetary_system::PlanetarySystem;
use crate::astronomy::satellite_systems::constraints::Constraints as SatelliteSystemsConstraints;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Host Star constraints.
  pub host_star_constraints: Option<HostStarConstraints>,
  /// Satellite Systems constraints.
  pub satellite_systems_constraints: Option<SatelliteSystemsConstraints>,
}

impl Constraints {
  /// Generate a habitable star subsystem.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let host_star_constraints = Some(HostStarConstraints::habitable());
    let satellite_systems_constraints = Some(SatelliteSystemsConstraints::habitable());
    let result = Self {
      host_star_constraints,
      satellite_systems_constraints,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<PlanetarySystem, Error> {
    trace_enter!();
    let host_star_constraints = self.host_star_constraints.unwrap_or_default();
    trace_var!(host_star_constraints);
    let satellite_systems_constraints = self.satellite_systems_constraints.unwrap_or_default();
    trace_var!(satellite_systems_constraints);
    let host_star = host_star_constraints.generate(rng)?;
    trace_var!(host_star);
    let satellite_systems = satellite_systems_constraints.generate(rng, &host_star)?;
    trace_var!(satellite_systems);
    let result = PlanetarySystem {
      host_star,
      satellite_systems,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let host_star_constraints = None;
    let satellite_systems_constraints = None;
    Self {
      host_star_constraints,
      satellite_systems_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let mut planetary_system = Constraints::habitable().generate(&mut rng)?;
    let mut is_habitable = planetary_system.is_habitable();
    let mut counter = 0;
    while !is_habitable && counter < 50 {
      planetary_system = Constraints::habitable().generate(&mut rng)?;
      is_habitable = planetary_system.is_habitable();
      counter += 1;
    }
    trace_var!(planetary_system);
    print_var!(planetary_system);
    planetary_system.check_habitable()?;
    assert!(planetary_system.is_habitable());
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let planetary_system = Constraints::default().generate(&mut rng)?;
    trace_var!(planetary_system);
    print_var!(planetary_system);
    trace_exit!();
    Ok(())
  }
}
