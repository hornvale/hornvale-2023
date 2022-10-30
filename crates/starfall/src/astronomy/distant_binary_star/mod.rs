use crate::astronomy::planetary_system::PlanetarySystem;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// A `DistantBinaryStar` is actually a pair of `PlanetarySystem` objects.
///
/// This may seem counterintuitive, but each member of a distant binary star
/// can itself be a binary star with its own orbiting planets.  A distant
/// binary star is thus very different in critical ways from a close binary
/// star, and we have to treat them as completely distinct although they
/// sound and might seem very similar.
///
/// And let's not get started on how disappointing it is to call something a
/// planetary system when it may not actually have any planets.  But I don't
/// think we have a better word or phrase for the idea.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DistantBinaryStar {
  /// The primary planetary system is the one with greater mass.
  pub primary: PlanetarySystem,
  /// The secondary planetary system has less mass.
  pub secondary: PlanetarySystem,
}

impl DistantBinaryStar {
  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    self.primary.check_habitable()?;
    self.secondary.check_habitable()?;
    let result = Ok(());
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
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

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_stellar_mass(&self) -> f64 {
    trace_enter!();
    let result = self.primary.get_stellar_mass() + self.secondary.get_stellar_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    let result = self.primary.get_stellar_count() + self.secondary.get_stellar_count();
    trace_var!(result);
    trace_exit!();
    result
  }
}
