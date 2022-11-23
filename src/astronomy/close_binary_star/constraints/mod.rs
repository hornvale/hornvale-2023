use crate::astronomy::_type::*;
use crate::astronomy::close_binary_star::constants::*;
use crate::astronomy::close_binary_star::error::Error;
use crate::astronomy::close_binary_star::CloseBinaryStar;
use crate::astronomy::star::constraints::Constraints as StarConstraints;
use rand::prelude::*;
use std::default::Default;

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
  pub minimum_combined_mass: Option<MSol>,
  /// The maximum combined mass of the stars, in Msol.
  pub maximum_combined_mass: Option<MSol>,
  /// The minimum individual mass of the stars, in Msol.
  pub minimum_individual_mass: Option<MSol>,
  /// The maximum individual mass of the stars, in Msol.
  pub maximum_individual_mass: Option<MSol>,
  /// The minimum separation between the stars, in AU.
  pub minimum_average_separation: Option<LAu>,
  /// The maximum separation between the stars, in AU.
  pub maximum_average_separation: Option<LAu>,
  /// The minimum orbital eccentricity (unitless).
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital_eccentricity (unitless).
  pub maximum_orbital_eccentricity: Option<f64>,
  /// The minimum age of the stars, in Gyr.
  pub minimum_age: Option<TGyr>,
  /// The maximum age of the stars, in Gyr.
  pub maximum_age: Option<TGyr>,
  /// Enforce habitability.
  pub enforce_habitability: bool,
  /// Star constraints.
  pub star_constraints: Option<StarConstraints>,
}

impl Constraints {
  /// Generate a habitable binary star.
  pub fn habitable() -> Self {
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
    Self {
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
    }
  }

  /// Generate a binary star from our constraints.
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<CloseBinaryStar, Error> {
    let mut minimum_combined_mass = self.minimum_combined_mass.unwrap_or(MINIMUM_COMBINED_MASS);
    let maximum_combined_mass = self.maximum_combined_mass.unwrap_or(MAXIMUM_COMBINED_MASS);
    let _minimum_individual_mass = self.minimum_individual_mass.unwrap_or(MINIMUM_INDIVIDUAL_MASS);
    let maximum_individual_mass = self.maximum_individual_mass.unwrap_or(MAXIMUM_INDIVIDUAL_MASS);
    let minimum_orbital_eccentricity = self
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = self
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_ORBITAL_ECCENTRICITY);
    let minimum_average_separation = self.minimum_average_separation.unwrap_or(MINIMUM_AVERAGE_SEPARATION);
    let maximum_average_separation = self.maximum_average_separation.unwrap_or(MAXIMUM_AVERAGE_SEPARATION);
    let orbital_eccentricity = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
    let average_separation = LAu(rng.gen_range(minimum_average_separation.0..maximum_average_separation.0));
    let combined_mass;
    let primary_mass;
    let secondary_mass;
    let mut primary_constraints;
    let mut secondary_constraints;
    if self.enforce_habitability {
      let bare_minimum =
        MSol((1.1 * (4.0 * maximum_average_separation.0 * (1.0 + orbital_eccentricity)).powf(2.0)).powf(1.0 / 4.0));
      if minimum_combined_mass < bare_minimum {
        minimum_combined_mass = MSol(1.1 * bare_minimum.0);
      }
      primary_constraints = self.star_constraints.unwrap_or_else(StarConstraints::habitable);
      secondary_constraints = self.star_constraints.unwrap_or_else(StarConstraints::habitable);
    } else {
      primary_constraints = self.star_constraints.unwrap_or_default();
      secondary_constraints = self.star_constraints.unwrap_or_default();
    }
    let (primary, secondary) = {
      combined_mass = MSol(rng.gen_range(minimum_combined_mass.0..maximum_combined_mass.0));
      let half = combined_mass / 2.0;
      let mut top = combined_mass - MINIMUM_HABITABLE_INDIVIDUAL_MASS;
      if self.enforce_habitability && top > maximum_individual_mass {
        top = maximum_individual_mass;
      }
      primary_mass = MSol(rng.gen_range(half.0..top.0));
      secondary_mass = combined_mass - primary_mass;
      primary_constraints.minimum_mass = Some(MSol(0.999 * primary_mass.0));
      primary_constraints.maximum_mass = Some(MSol(1.001 * primary_mass.0));
      secondary_constraints.minimum_mass = Some(MSol(0.999 * secondary_mass.0));
      secondary_constraints.maximum_mass = Some(MSol(1.001 * secondary_mass.0));
      let mut primary = primary_constraints.generate(rng)?;
      let mut secondary = secondary_constraints.generate(rng)?;
      let minimum_age = match self.enforce_habitability {
        true => MINIMUM_HABITABLE_AGE,
        false => TGyr(0.1 * primary.life_expectancy.0),
      };
      let maximum_age = TGyr(0.9 * primary.life_expectancy.0);
      let current_age = TGyr(rng.gen_range(minimum_age.0..maximum_age.0));
      primary.current_age = current_age;
      secondary.current_age = current_age;
      (primary, secondary)
    };
    let result = CloseBinaryStar::from_stars(rng, primary, secondary, average_separation, orbital_eccentricity)?;
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
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
    Self {
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
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_default() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let binary = &Constraints::default().generate(&mut rng)?;
    print_var!(binary);
    Ok(())
  }

  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let binary = &Constraints::habitable().generate(&mut rng)?;
    print_var!(binary);
    Ok(())
  }

  #[test]
  pub fn test_default_bulk() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
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
    Ok(())
  }

  #[test]
  pub fn test_habitable_bulk() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
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
    Ok(())
  }
}
