use rand::prelude::*;
use std::default::Default;

use crate::astronomy::close_binary_star::constants::*;
use crate::astronomy::close_binary_star::error::Error;
use crate::astronomy::close_binary_star::CloseBinaryStar;
use crate::astronomy::star::constraints::Constraints as StarConstraints;

/// Constraints for creating a binary star.
///
/// As it turns out, randomly generating a habitable binary star is HARD.
///
/// The dwarf stars that are best for habitability cause an inward pressure
/// on the habitable zone, but the gravitational field of the stars is very
/// dangerous and chips away at the habitable zone from inside.
///
/// After beating my head against this for a while, I think the best approach
/// is to contort the parameters to give a high rate of success, while sadly
/// acknowledging that a lot of the potential variety has been squashed :(
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum combined mass of the stars, in Msol.
  pub minimum_combined_mass: Option<f64>,
  /// The maximum combined mass of the stars, in Msol.
  pub maximum_combined_mass: Option<f64>,
  /// The minimum individual mass of the stars, in Msol.
  pub minimum_individual_mass: Option<f64>,
  /// The maximum individual mass of the stars, in Msol.
  pub maximum_individual_mass: Option<f64>,
  /// The minimum separation between the stars, in Msol.
  pub minimum_average_separation: Option<f64>,
  /// The maximum separation between the stars, in Msol.
  pub maximum_average_separation: Option<f64>,
  /// The minimum orbital eccentricity.
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital_eccentricity.
  pub maximum_orbital_eccentricity: Option<f64>,
  /// The minimum age of the stars, in Gyr.
  pub minimum_age: Option<f64>,
  /// The maximum age of the stars, in Gyr.
  pub maximum_age: Option<f64>,
  /// Enforce habitability.
  pub enforce_habitability: bool,
  /// Star constraints.
  pub star_constraints: Option<StarConstraints>,
}

impl Constraints {
  /// Generate a habitable binary star.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let minimum_combined_mass = Some(MINIMUM_HABITABLE_COMBINED_MASS);
    let maximum_combined_mass = Some(MAXIMUM_HABITABLE_COMBINED_MASS);
    let minimum_individual_mass = Some(MINIMUM_HABITABLE_INDIVIDUAL_MASS);
    let maximum_individual_mass = Some(MAXIMUM_HABITABLE_INDIVIDUAL_MASS);
    let minimum_orbital_eccentricity = Some(MINIMUM_HABITABLE_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_HABITABLE_ORBITAL_ECCENTRICITY);
    let maximum_average_separation = Some(MAXIMUM_HABITABLE_AVERAGE_SEPARATION);
    let minimum_age = Some(MINIMUM_HABITABLE_AGE);
    let enforce_habitability = true;
    let star_constraints = Some(StarConstraints::habitable());
    let result = Self {
      minimum_combined_mass,
      maximum_combined_mass,
      minimum_individual_mass,
      maximum_individual_mass,
      maximum_average_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      minimum_age,
      enforce_habitability,
      star_constraints,
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate a binary star from our constraints.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<CloseBinaryStar, Error> {
    trace_enter!();
    let mut minimum_combined_mass = self.minimum_combined_mass.unwrap_or(MINIMUM_COMBINED_MASS);
    trace_var!(minimum_combined_mass);
    let maximum_combined_mass = self.maximum_combined_mass.unwrap_or(MAXIMUM_COMBINED_MASS);
    trace_var!(maximum_combined_mass);
    let minimum_individual_mass = self.minimum_individual_mass.unwrap_or(MINIMUM_INDIVIDUAL_MASS);
    trace_var!(minimum_individual_mass);
    let maximum_individual_mass = self.maximum_individual_mass.unwrap_or(MAXIMUM_INDIVIDUAL_MASS);
    trace_var!(maximum_individual_mass);
    let minimum_orbital_eccentricity = self
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(minimum_orbital_eccentricity);
    let maximum_orbital_eccentricity = self
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(maximum_orbital_eccentricity);
    let minimum_average_separation = self.minimum_average_separation.unwrap_or(MINIMUM_AVERAGE_SEPARATION);
    trace_var!(minimum_average_separation);
    let maximum_average_separation = self.maximum_average_separation.unwrap_or(MAXIMUM_AVERAGE_SEPARATION);
    trace_var!(maximum_average_separation);
    let orbital_eccentricity = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
    trace_var!(orbital_eccentricity);
    let average_separation = rng.gen_range(minimum_average_separation..maximum_average_separation);
    trace_var!(average_separation);
    let combined_mass;
    let primary_mass;
    let secondary_mass;
    let mut primary_constraints;
    let mut secondary_constraints;
    if self.enforce_habitability {
      let bare_minimum =
        (1.1 * (4.0 * maximum_average_separation * (1.0 + orbital_eccentricity)).powf(2.0)).powf(1.0 / 4.0);
      if minimum_combined_mass < bare_minimum {
        minimum_combined_mass = 1.1 * bare_minimum;
      }
      primary_constraints = self.star_constraints.unwrap_or(StarConstraints::habitable());
      secondary_constraints = self.star_constraints.unwrap_or(StarConstraints::habitable());
    } else {
      primary_constraints = self.star_constraints.unwrap_or(StarConstraints::default());
      secondary_constraints = self.star_constraints.unwrap_or(StarConstraints::default());
    }
    let (primary, secondary) = {
      combined_mass = rng.gen_range(minimum_combined_mass..maximum_combined_mass);
      let half = combined_mass / 2.0;
      let mut top = combined_mass - MINIMUM_HABITABLE_INDIVIDUAL_MASS;
      if self.enforce_habitability && top > maximum_individual_mass {
        top = maximum_individual_mass;
      }
      primary_mass = rng.gen_range(half..top);
      secondary_mass = combined_mass - primary_mass;
      primary_constraints.minimum_mass = Some(0.999 * primary_mass);
      primary_constraints.maximum_mass = Some(1.001 * primary_mass);
      secondary_constraints.minimum_mass = Some(0.999 * secondary_mass);
      secondary_constraints.maximum_mass = Some(1.001 * secondary_mass);
      let mut primary = primary_constraints.generate(rng)?;
      let mut secondary = secondary_constraints.generate(rng)?;
      let minimum_age = match self.enforce_habitability {
        true => MINIMUM_HABITABLE_AGE,
        false => 0.1 * primary.life_expectancy,
      };
      trace_var!(minimum_age);
      let maximum_age = 0.9 * primary.life_expectancy;
      trace_var!(maximum_age);
      let current_age = rng.gen_range(minimum_age..maximum_age);
      trace_var!(current_age);
      primary.current_age = current_age;
      secondary.current_age = current_age;
      (primary, secondary)
    };
    trace_var!(primary);
    trace_var!(secondary);
    let result = CloseBinaryStar::from_stars(rng, primary, secondary, average_separation, orbital_eccentricity)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  #[named]
  fn default() -> Self {
    trace_enter!();
    let minimum_combined_mass = Some(MINIMUM_COMBINED_MASS);
    let maximum_combined_mass = Some(MAXIMUM_COMBINED_MASS);
    let minimum_individual_mass = Some(MINIMUM_INDIVIDUAL_MASS);
    let maximum_individual_mass = Some(MAXIMUM_INDIVIDUAL_MASS);
    let minimum_average_separation = None;
    let maximum_average_separation = None;
    let minimum_orbital_eccentricity = Some(MINIMUM_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_ORBITAL_ECCENTRICITY);
    let minimum_age = None;
    let maximum_age = None;
    let enforce_habitability = false;
    let star_constraints = None;
    let result = Self {
      minimum_combined_mass,
      maximum_combined_mass,
      minimum_individual_mass,
      maximum_individual_mass,
      minimum_average_separation,
      maximum_average_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      minimum_age,
      maximum_age,
      enforce_habitability,
      star_constraints,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_default() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let binary = &Constraints::default().generate(&mut rng)?;
    trace_var!(binary);
    print_var!(binary);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let binary = &Constraints::habitable().generate(&mut rng)?;
    trace_var!(binary);
    print_var!(binary);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_default_bulk() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let mut success = 0;
    let trials = 1000;
    let mut counter = 0;
    loop {
      match &Constraints::default().generate(&mut rng) {
        Ok(_binary) => success += 1,
        Err(error) => print_var!(error),
      }
      counter += 1;
      if counter >= trials {
        break;
      }
    }
    print_var!(success);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable_bulk() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let mut success = 0;
    let trials = 1000;
    let mut counter = 0;
    loop {
      match &Constraints::habitable().generate(&mut rng) {
        Ok(_binary) => success += 1,
        Err(error) => print_var!(error),
      }
      counter += 1;
      if counter >= trials {
        break;
      }
    }
    print_var!(success);
    assert_eq!(counter, trials);
    trace_exit!();
    Ok(())
  }
}
