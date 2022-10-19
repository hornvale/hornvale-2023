use rand::prelude::*;
use std::default::Default;

use crate::astronomy::distant_binary_star::constraints::Constraints as DistantBinaryStarConstraints;
use crate::astronomy::planetary_system::constraints::Constraints as PlanetarySystemConstraints;
use crate::astronomy::star_subsystem::constants::*;
use crate::astronomy::star_subsystem::error::Error;
use crate::astronomy::star_subsystem::StarSubsystem;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The probability that we generate a distant binary star.
  pub distant_binary_probability: Option<f64>,
  /// Distant Binary Star constraints.
  pub distant_binary_star_constraints: Option<DistantBinaryStarConstraints>,
  /// Planetary System constraints.
  pub planetary_system_constraints: Option<PlanetarySystemConstraints>,
}

impl Constraints {
  /// Generate a habitable star subsystem.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let result = Self {
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<StarSubsystem, Error> {
    trace_enter!();
    use StarSubsystem::*;
    let distant_binary_probability = self.distant_binary_probability.unwrap_or(DISTANT_BINARY_PROBABILITY);
    trace_var!(distant_binary_probability);
    let generate_planetary_system: bool = rng.gen_range(0.0..1.0) > distant_binary_probability;
    trace_var!(generate_planetary_system);
    let result = if generate_planetary_system {
      let planetary_system_constraints = self.planetary_system_constraints.unwrap_or_default();
      trace_var!(planetary_system_constraints);
      PlanetarySystem(planetary_system_constraints.generate(rng)?)
    } else {
      let distant_binary_star_constraints = self.distant_binary_star_constraints.unwrap_or_default();
      trace_var!(distant_binary_star_constraints);
      DistantBinaryStar(distant_binary_star_constraints.generate(rng)?)
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let distant_binary_probability = Some(DISTANT_BINARY_PROBABILITY);
    let distant_binary_star_constraints = None;
    let planetary_system_constraints = None;
    Self {
      distant_binary_probability,
      distant_binary_star_constraints,
      planetary_system_constraints,
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
    let star_subsystem = &Constraints::default().generate(&mut rng)?;
    trace_var!(star_subsystem);
    print_var!(star_subsystem);
    trace_exit!();
    Ok(())
  }
}
