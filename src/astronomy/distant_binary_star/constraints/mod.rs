use rand::prelude::*;
use std::default::Default;

use crate::astronomy::distant_binary_star::constants::*;
use crate::astronomy::distant_binary_star::error::Error;
use crate::astronomy::distant_binary_star::DistantBinaryStar;
use crate::astronomy::planetary_system::constraints::Constraints as PlanetarySystemConstraints;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum average separation, in AU.
  pub minimum_average_separation: Option<f64>,
  /// The maximum average separation, in AU.
  pub maximum_average_separation: Option<f64>,
  /// The minimum orbital eccentricity.
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital eccentricity.
  pub maximum_orbital_eccentricity: Option<f64>,
}

impl Constraints {
  /// Generate a distant binary star with at least one habitable system.

  pub fn habitable() -> Self {
    Self {
      ..Constraints::default()
    }
  }

  /// Generate.

  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<DistantBinaryStar, Error> {
    let primary_constraints = PlanetarySystemConstraints::default();
    let primary = primary_constraints.generate(rng)?;
    let secondary_constraints = PlanetarySystemConstraints::default();
    let secondary = secondary_constraints.generate(rng)?;
    let result = DistantBinaryStar { primary, secondary };

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_average_separation = Some(MINIMUM_AVERAGE_SEPARATION);
    let maximum_average_separation = Some(MAXIMUM_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_ORBITAL_ECCENTRICITY);
    Self {
      minimum_average_separation,
      maximum_average_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
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
    let distant_binary_star = Constraints::default().generate(&mut rng)?;

    print_var!(distant_binary_star);

    Ok(())
  }

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let distant_binary_star = Constraints::habitable().generate(&mut rng)?;

    print_var!(distant_binary_star);
    distant_binary_star.get_stellar_mass();
    distant_binary_star.is_habitable();

    Ok(())
  }
}
