use crate::astronomy::_type::*;
use crate::astronomy::host_star::HostStar;
use crate::astronomy::moon::constraints::Constraints as MoonConstraints;
use crate::astronomy::moons::constants::*;
use crate::astronomy::moons::error::Error;
use crate::astronomy::moons::Moons;
use crate::astronomy::planet::Planet;
use rand::prelude::*;

/// Constraints for creating a moon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// A constraint for moons.
  pub moon_constraints: Option<MoonConstraints>,
}

impl Constraints {
  /// Generate.
  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    host_star: &HostStar,
    star_distance: LAu,
    planet: &Planet,
  ) -> Result<Moons, Error> {
    let minimum_count;
    let maximum_count;
    use Planet::*;
    match planet {
      TerrestrialPlanet(_) => {
        minimum_count = MINIMUM_TERRESTRIAL_MOONS;
        maximum_count = MAXIMUM_TERRESTRIAL_MOONS;
      },
      GasGiantPlanet(_) => {
        minimum_count = MINIMUM_GAS_GIANT_MOONS;
        maximum_count = MAXIMUM_GAS_GIANT_MOONS;
      },
    }
    let moon_constraints = self.moon_constraints.unwrap_or_default();
    let rocky_moon_density = 3.35;
    let satellite_zone = {
      let inner = 2.44 * planet.get_radius() * 6_371.0 * (planet.get_density() / rocky_moon_density).powf(1.0 / 3.0);
      // @todo: improve this.
      let outer = 20.0 * inner;
      (inner, outer)
    };
    let moons = {
      let count = rng.gen_range(minimum_count..=maximum_count);
      let mut moons = vec![];
      for _ in 1..count {
        let planet_distance = LKm(rng.gen_range(satellite_zone.0..satellite_zone.1));
        let moon = moon_constraints.generate(rng, host_star, star_distance, planet, planet_distance)?;
        moons.push(moon);
      }
      moons
    };
    let result = Moons { moons };
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let moon_constraints = None;
    Self { moon_constraints }
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
    let host_star = &HostStarConstraints::default().generate(&mut rng)?;
    let habitable_zone = host_star.get_habitable_zone();
    let star_distance = LAu(rng.gen_range(habitable_zone.0..habitable_zone.1));
    let planet = &PlanetConstraints::default().generate(&mut rng, host_star, star_distance.0)?;
    let moon = &Constraints::default().generate(&mut rng, host_star, star_distance, planet)?;
    print_var!(moon);
    Ok(())
  }
}
