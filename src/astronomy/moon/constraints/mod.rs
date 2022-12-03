use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;
use crate::astronomy::host_star::HostStar;
use crate::astronomy::moon::error::Error;
use crate::astronomy::moon::Moon;
use crate::astronomy::planet::Planet;
use rand::prelude::*;

/// Constraints for creating a moon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass, in MLuna.
  pub minimum_mass: Option<MLuna>,
  /// The maximum mass, in MLuna.
  pub maximum_mass: Option<MLuna>,
}

impl Constraints {
  /// Generate.
  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    host_star: &HostStar,
    star_distance: LAu,
    planet: &Planet,
    planet_distance: LKm,
  ) -> Result<Moon, Error> {
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MOON_MASS);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MOON_MASS);
    let mass = MLuna(rng.gen_range(minimum_mass.0..maximum_mass.0));
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
    let star_distance = LAu(rng.gen_range(habitable_zone.0 .0..habitable_zone.1 .0));
    let planet = &PlanetConstraints::habitable().generate(&mut rng, host_star, star_distance)?;
    let moon = &Constraints::default().generate(&mut rng, host_star, star_distance, planet, LKm(400_000.0))?;
    print_var!(moon);
    Ok(())
  }
}
