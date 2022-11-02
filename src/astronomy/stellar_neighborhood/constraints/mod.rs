use rand::prelude::*;
use std::f64::consts::PI;

use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::stellar_neighbor::constraints::Constraints as StellarNeighborConstraints;
use crate::astronomy::stellar_neighborhood::constants::*;
use crate::astronomy::stellar_neighborhood::error::*;
use crate::astronomy::stellar_neighborhood::StellarNeighborhood;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// The density of the neighborhood, in stars per cubic light year.
  pub density: Option<f64>,
  /// Any constraints placed on the various neighbors.
  pub neighbor_constraints: Option<StellarNeighborConstraints>,
}

impl Constraints {
  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let neighbor_constraints = Some(StellarNeighborConstraints::habitable());
    Self {
      neighbor_constraints,
      ..Constraints::default()
    }
  }

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.

  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<StellarNeighborhood, Error> {
    let radius = self.radius.unwrap_or(STELLAR_NEIGHBORHOOD_RADIUS);

    let density = self.density.unwrap_or(STELLAR_NEIGHBORHOOD_DENSITY);

    let volume = (4.0 / 3.0) * PI * radius.powf(3.0);

    let average_stars = density * volume;

    let number_of_stars = rng.gen_range((0.875 * average_stars)..(1.125 * average_stars)) as usize;

    let mut neighbors = vec![];

    let mut star_count = 0;
    let neighbor_constraints = self.neighbor_constraints.unwrap_or(StellarNeighborConstraints {
      radius: Some(radius),
      system_constraints: Some(StarSystemConstraints::default()),
    });

    loop {
      let neighbor = neighbor_constraints.generate(rng)?;
      star_count += neighbor.get_stellar_count() as usize;
      neighbors.push(neighbor);
      if star_count >= number_of_stars {
        break;
      }
    }

    let result = StellarNeighborhood {
      radius,
      density,
      neighbors,
      star_count,
    };

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(STELLAR_NEIGHBORHOOD_RADIUS);
    let density = Some(STELLAR_NEIGHBORHOOD_DENSITY);
    let neighbor_constraints = Some(StellarNeighborConstraints::default());
    Self {
      radius,
      density,
      neighbor_constraints,
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

    let constraints = Constraints::default();
    let stellar_neighborhood = constraints.generate(&mut rng)?;
    info_var!(stellar_neighborhood);
    print_var!(stellar_neighborhood);

    Ok(())
  }

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();

    let mut rng = thread_rng();

    let constraints = Constraints::habitable();
    let stellar_neighborhood = constraints.generate(&mut rng)?;
    info_var!(stellar_neighborhood);
    print_var!(stellar_neighborhood);

    Ok(())
  }
}
