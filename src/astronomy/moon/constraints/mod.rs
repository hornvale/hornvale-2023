use rand::prelude::*;

use crate::astronomy::host_star::HostStar;
use crate::astronomy::moon::constants::*;
use crate::astronomy::moon::error::Error;
use crate::astronomy::moon::Moon;
use crate::astronomy::planet::Planet;

/// Constraints for creating a moon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass, in Mmoon.
  pub minimum_mass: Option<f64>,
  /// The maximum mass, in Mmoon.
  pub maximum_mass: Option<f64>,
}

impl Constraints {
  /// Generate.

  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    host_star: &HostStar,
    star_distance: f64,
    planet: &Planet,
    planet_distance: f64,
  ) -> Result<Moon, Error> {
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    let mass = rng.gen_range(minimum_mass..maximum_mass);
    let result = Moon::from_environment(mass, host_star, star_distance, planet, planet_distance)?;

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    Self {
      minimum_mass,
      maximum_mass,
    }
  }
}

#[cfg(test)]
pub mod test {

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
  use crate::astronomy::planet::constraints::Constraints as PlanetConstraints;
  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let host_star = &HostStarConstraints::habitable().generate(&mut rng)?;
    let habitable_zone = host_star.get_habitable_zone();
    let star_distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    let planet = &PlanetConstraints::habitable().generate(&mut rng, host_star, star_distance)?;
    let moon = &Constraints::default().generate(&mut rng, host_star, star_distance, planet, 400_000.0)?;

    print_var!(moon);

    Ok(())
  }
}
