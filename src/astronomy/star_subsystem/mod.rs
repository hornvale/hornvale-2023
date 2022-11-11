use crate::astronomy::distant_binary_star::DistantBinaryStar;
use crate::astronomy::planetary_system::PlanetarySystem;

pub mod constants;
pub mod constraints;
pub mod error;
use error::*;

/// The `StarSubsystem` type.
///
/// A StarSubsystem is either one star with a planetary system or a distant binary.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum StarSubsystem {
  /// A distant binary system.
  DistantBinaryStar(DistantBinaryStar),
  /// Any other planetary system.
  PlanetarySystem(PlanetarySystem),
}

impl StarSubsystem {
  /// Indicate whether this star is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
    use StarSubsystem::*;

    match &self {
      DistantBinaryStar(distant_binary_star) => Ok(distant_binary_star.check_habitable()?),
      PlanetarySystem(planetary_system) => Ok(planetary_system.check_habitable()?),
    }
  }

  /// Indicate whether this star is capable of supporting conventional life.
  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  pub fn get_stellar_mass(&self) -> f64 {
    use StarSubsystem::*;

    match &self {
      DistantBinaryStar(distant_binary_star) => distant_binary_star.get_stellar_mass(),
      PlanetarySystem(planetary_system) => planetary_system.get_stellar_mass(),
    }
  }

  /// Retrieve or calculate the total number of stars in the system.
  pub fn get_stellar_count(&self) -> u8 {
    use StarSubsystem::*;

    match &self {
      DistantBinaryStar(distant_binary_star) => distant_binary_star.get_stellar_count(),
      PlanetarySystem(planetary_system) => planetary_system.get_stellar_count(),
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[test]
  pub fn get_random() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let constraints = Constraints::default();
    let result = constraints.generate(&mut rng)?;

    print_var!(result);

    Ok(())
  }
}
