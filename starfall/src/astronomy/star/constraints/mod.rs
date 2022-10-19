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
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Star, Error> {
    trace_enter!();
    let mass = match self.make_habitable {
      true => get_random_habitable_stellar_mass(rng),
      false => get_random_stellar_mass(rng),
    };
    trace_var!(mass);
    let mut result = Star::from_mass(rng, mass)?;
    trace_var!(result);
    let minimum_age = match self.make_habitable {
      true => MINIMUM_HABITABLE_AGE,
      false => 0.1 * result.life_expectancy,
    };
    trace_var!(minimum_age);
    let maximum_age = 0.9 * result.life_expectancy;
    trace_var!(maximum_age);
    result.current_age = rng.gen_range(minimum_age..maximum_age);
    trace_var!(result);
    trace_exit!();
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

  #[named]
  #[test]
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Constraints::default().generate(&mut rng)?;
    trace_var!(star);
    print_var!(star);
    trace_exit!();
    Ok(())
  }
}
