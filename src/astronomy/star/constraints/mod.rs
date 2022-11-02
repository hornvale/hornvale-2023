use rand::prelude::*;
use std::default::Default;

use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::mass::*;
use crate::astronomy::star::Star;

/// Constraints for creating a main-sequence star.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Minimum amount of mass.
  pub minimum_mass: Option<f64>,
  /// Maximum amount of mass.
  pub maximum_mass: Option<f64>,
  /// Ensure this star is habitable.
  pub make_habitable: bool,
}

impl Constraints {
  /// Generate a habitable star.
  pub fn habitable() -> Self {
    let minimum_mass = Some(MINIMUM_HABITABLE_MASS);
    let maximum_mass = Some(MAXIMUM_HABITABLE_MASS);
    let make_habitable = true;
    Self {
      minimum_mass,
      maximum_mass,
      make_habitable,
    }
  }

  /// Generate.

  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Star, Error> {
    let mass = match self.make_habitable {
      true => get_random_habitable_stellar_mass(rng),
      false => get_random_stellar_mass(rng),
    };

    let mut result = Star::from_mass(rng, mass)?;

    let minimum_age = match self.make_habitable {
      true => MINIMUM_HABITABLE_AGE,
      false => 0.1 * result.life_expectancy,
    };

    let maximum_age = 0.9 * result.life_expectancy;

    result.current_age = rng.gen_range(minimum_age..maximum_age);

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    let make_habitable = false;
    Self {
      minimum_mass,
      maximum_mass,
      make_habitable,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[test]
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();

    let mut rng = thread_rng();

    let star = Constraints::default().generate(&mut rng)?;

    print_var!(star);

    Ok(())
  }

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();

    let mut rng = thread_rng();

    let star = Constraints::habitable().generate(&mut rng)?;

    print_var!(star);
    assert!(star.is_habitable());

    Ok(())
  }
}
