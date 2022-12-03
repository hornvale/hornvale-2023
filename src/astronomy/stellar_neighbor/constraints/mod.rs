use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;
use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::stellar_neighbor::error::Error;
use crate::astronomy::stellar_neighbor::math::point::get_random_point_in_sphere;
use crate::astronomy::stellar_neighbor::StellarNeighbor;
use rand::prelude::*;
use std::default::Default;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<LLyr>,
  /// Star system constraints.
  pub system_constraints: Option<StarSystemConstraints>,
}

impl Constraints {
  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let system_constraints = Some(StarSystemConstraints::habitable());
    Self {
      system_constraints,
      ..Constraints::default()
    }
  }

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<StellarNeighbor, Error> {
    // @todo: move this into stellar neighborhood, probably.
    let radius = self.radius.unwrap_or(STELLAR_NEIGHBORHOOD_RADIUS);
    let raw_coordinates = get_random_point_in_sphere(rng);
    let x = raw_coordinates.0 * radius.0;
    let y = raw_coordinates.1 * radius.0;
    let z = raw_coordinates.2 * radius.0;
    let coordinates = (x, y, z);
    let distance = LLyr((x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt());
    let system_constraints = self.system_constraints.unwrap_or_default();
    let star_system = system_constraints.generate(rng)?;
    let name = star_system.name.clone();
    let result = StellarNeighbor {
      coordinates,
      star_system,
      distance,
      name,
    };
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(STELLAR_NEIGHBORHOOD_RADIUS);
    let system_constraints = Some(StarSystemConstraints::default());
    Self {
      radius,
      system_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let stellar_neighbor = &Constraints::default().generate(&mut rng)?;
    print_var!(stellar_neighbor);
    Ok(())
  }

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let stellar_neighbor = &Constraints::habitable().generate(&mut rng)?;
    print_var!(stellar_neighbor);
    Ok(())
  }
}
