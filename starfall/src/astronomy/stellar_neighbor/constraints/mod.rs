use rand::prelude::*;
use std::default::Default;

use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::stellar_neighbor::error::Error;
use crate::astronomy::stellar_neighbor::math::point::get_random_point_in_sphere;
use crate::astronomy::stellar_neighbor::StellarNeighbor;
use crate::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_RADIUS;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// Star system constraints.
  pub system_constraints: Option<StarSystemConstraints>,
}

impl Constraints {
  /// Generate a habitable star system.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let system_constraints = Some(StarSystemConstraints::habitable());
    trace_var!(system_constraints);
    Self {
      system_constraints,
      ..Constraints::default()
    }
  }

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<StellarNeighbor, Error> {
    trace_enter!();
    // @todo: move this into stellar neighborhood, probably.
    let radius = self.radius.unwrap_or(STELLAR_NEIGHBORHOOD_RADIUS);
    trace_var!(radius);
    let raw_coordinates = get_random_point_in_sphere(rng);
    trace_var!(raw_coordinates);
    let x = raw_coordinates.0 * radius;
    trace_var!(x);
    let y = raw_coordinates.1 * radius;
    trace_var!(y);
    let z = raw_coordinates.2 * radius;
    trace_var!(z);
    let coordinates = (x, y, z);
    trace_var!(coordinates);
    let distance = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();
    let system_constraints = self.system_constraints.unwrap_or(StarSystemConstraints::default());
    let star_system = system_constraints.generate(rng)?;
    trace_var!(star_system);
    let name = star_system.name.clone();
    let result = StellarNeighbor {
      coordinates,
      star_system,
      distance,
      name,
    };
    trace_var!(result);
    trace_exit!();
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

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let stellar_neighbor = &Constraints::default().generate(&mut rng)?;
    trace_var!(stellar_neighbor);
    print_var!(stellar_neighbor);
    trace_exit!();
    Ok(())
  }
}
