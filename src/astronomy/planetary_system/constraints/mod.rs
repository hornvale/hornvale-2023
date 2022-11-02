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

  pub fn habitable() -> Self {
    let host_star_constraints = Some(HostStarConstraints::habitable());
    let satellite_systems_constraints = Some(SatelliteSystemsConstraints::habitable());

    Self {
      host_star_constraints,
      satellite_systems_constraints,
    }
  }

  /// Generate.

  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<PlanetarySystem, Error> {
    let host_star_constraints = self.host_star_constraints.unwrap_or_default();
    let satellite_systems_constraints = self.satellite_systems_constraints.unwrap_or_default();
    let host_star = host_star_constraints.generate(rng)?;
    let satellite_systems = satellite_systems_constraints.generate(rng, &host_star)?;
    let result = PlanetarySystem {
      host_star,
      satellite_systems,
    };

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

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let mut planetary_system = Constraints::habitable().generate(&mut rng)?;
    let mut is_habitable = planetary_system.is_habitable();
    let mut counter = 0;
    while !is_habitable && counter < 50 {
      planetary_system = Constraints::habitable().generate(&mut rng)?;
      is_habitable = planetary_system.is_habitable();
      counter += 1;
    }

    print_var!(planetary_system);
    planetary_system.check_habitable()?;
    assert!(planetary_system.is_habitable());

    Ok(())
  }

  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let planetary_system = Constraints::default().generate(&mut rng)?;

    print_var!(planetary_system);

    Ok(())
  }
}
