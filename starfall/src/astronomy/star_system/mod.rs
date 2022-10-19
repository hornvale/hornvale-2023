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
#[derive(Clone, Debug, PartialEq)]
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
  #[named]
  pub fn get_stellar_mass(&self) -> f64 {
    trace_enter!();
    let result = self.star_subsystem.get_stellar_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    let result = self.star_subsystem.get_stellar_count();
    trace_u8!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star system is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    let result = Ok(self.star_subsystem.check_habitable()?);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star system is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star_system = Constraints::habitable().generate(&mut rng)?;
    info_var!(star_system);
    print_var!(star_system);
    star_system.get_stellar_mass();
    star_system.is_habitable();
    trace_exit!();
    Ok(())
  }
}
