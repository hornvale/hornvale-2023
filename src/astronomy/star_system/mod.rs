use crate::astronomy::star_subsystem::StarSubsystem;

pub mod constraints;
pub mod error;
use error::*;

/// The `StarSystem` type.
///
/// This is probably a good place to include some notes on terminology.
///
/// For ease of programming, I'm conflating the concepts of "star" or "stellar"
/// systems and "planetary" systems.
///
/// Here, a "star system" means one or more stars gravitationally connected in
/// some interesting way, along with all of the planets and other satellites
/// bound to those stars in some interesting way.
///
/// And I use "solar system" only to refer to our (your and my) star system.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StarSystem {
  /// The basic configuration of the host star(s).
  pub star_subsystem: StarSubsystem,
  /// The name of the primary star.
  pub name: String,
}

impl StarSystem {
  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  pub fn get_stellar_mass(&self) -> f64 {
    self.star_subsystem.get_stellar_mass()
  }

  /// Retrieve or calculate the total number of stars in the system.
  pub fn get_stellar_count(&self) -> u8 {
    self.star_subsystem.get_stellar_count()
  }

  /// Indicate whether this star system is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
    Ok(self.star_subsystem.check_habitable()?)
  }

  /// Indicate whether this star system is capable of supporting conventional life.
  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
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
    let star_system = Constraints::habitable().generate(&mut rng)?;
    info_var!(star_system);
    print_var!(star_system);
    star_system.get_stellar_mass();
    star_system.is_habitable();

    Ok(())
  }
}
