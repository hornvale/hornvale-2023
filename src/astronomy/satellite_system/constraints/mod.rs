use rand::prelude::*;

use crate::astronomy::host_star::HostStar;
use crate::astronomy::moons::constraints::Constraints as MoonsConstraints;
use crate::astronomy::planet::constraints::Constraints as PlanetConstraints;
use crate::astronomy::satellite_system::error::Error;
use crate::astronomy::satellite_system::SatelliteSystem;

/// Constraints for creating a planet and its moons.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Planet constraints.
  pub planet_constraints: Option<PlanetConstraints>,
  /// Moons constraints.
  pub moons_constraints: Option<MoonsConstraints>,
}

impl Constraints {
  /// No constraints, just let it all hang out.
  pub fn habitable() -> Self {
    let planet_constraints = Some(PlanetConstraints::habitable());
    Self {
      planet_constraints,
      ..Constraints::default()
    }
  }

  /// Generate.

  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    host_star: &HostStar,
    star_distance: f64,
  ) -> Result<SatelliteSystem, Error> {
    let planet_constraints = self.planet_constraints.unwrap_or_default();

    let moons_constraints = self.moons_constraints.unwrap_or_default();

    let planet = planet_constraints.generate(rng, host_star, star_distance)?;

    let moons = moons_constraints.generate(rng, host_star, star_distance, &planet)?;

    let result = SatelliteSystem { planet, moons };

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let planet_constraints = None;
    let moons_constraints = None;
    Self {
      planet_constraints,
      moons_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();

    let mut rng = thread_rng();

    let host_star = HostStarConstraints::habitable().generate(&mut rng)?;
    let habitable_zone = host_star.get_habitable_zone();
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    let satellite_system = &Constraints::default().generate(&mut rng, &host_star, distance)?;

    print_var!(satellite_system);

    Ok(())
  }
}
