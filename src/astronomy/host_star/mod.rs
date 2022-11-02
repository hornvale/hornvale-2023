use crate::astronomy::close_binary_star::CloseBinaryStar;
use crate::astronomy::star::Star;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// A `HostStar` is either a `Star` or a `CloseBinaryStar`.
///
/// This may seem counterintuitive, but a `CloseBinaryStar` is actually more
/// closely related to a `Star` than a `DistantBinaryStar`.  The reason for
/// this is that habitable planets can be in a circumbinary orbit around a
/// `CloseBinaryStar`, but can only be in an orbit around one member of a
/// `DistantBinaryStar`.  As a result, we handle `DistantBinaryStar` objects
/// with a distinct class.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HostStar {
  /// A single star.
  Star(Box<Star>),
  /// A close binary star.
  CloseBinaryStar(Box<CloseBinaryStar>),
}

impl HostStar {
  /// Retrieve or calculate the age of the stars.
  ///
  /// Calculated in Gyr.

  pub fn get_current_age(&self) -> f64 {
    use HostStar::*;

    match &self {
      Star(star) => star.current_age,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_current_age(),
    }
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.

  pub fn get_stellar_mass(&self) -> f64 {
    use HostStar::*;

    match &self {
      Star(star) => star.mass,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_stellar_mass(),
    }
  }

  /// Retrieve or calculate the total number of stars in the system.

  pub fn get_stellar_count(&self) -> u8 {
    use HostStar::*;

    match &self {
      Star(_) => 1,
      CloseBinaryStar(_) => 2,
    }
  }

  /// Retrieve or calculate the frost line.

  pub fn get_frost_line(&self) -> f64 {
    use HostStar::*;

    match &self {
      Star(star) => star.frost_line,
      CloseBinaryStar(close_binary_star) => close_binary_star.frost_line,
    }
  }

  /// Retrieve or calculate the habitable zone.

  pub fn get_habitable_zone(&self) -> (f64, f64) {
    use HostStar::*;

    match &self {
      Star(star) => star.habitable_zone,
      CloseBinaryStar(close_binary_star) => close_binary_star.habitable_zone,
    }
  }

  /// Retrieve or calculate the satellite zone.

  pub fn get_satellite_zone(&self) -> (f64, f64) {
    use HostStar::*;

    match &self {
      Star(star) => star.satellite_zone,
      CloseBinaryStar(close_binary_star) => close_binary_star.satellite_zone,
    }
  }

  /// Retrieve or calculate the luminosity.

  pub fn get_luminosity(&self) -> f64 {
    use HostStar::*;

    match &self {
      Star(star) => star.luminosity,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_luminosity(),
    }
  }

  /// Indicate whether this star is capable of supporting conventional life.

  pub fn check_habitable(&self) -> Result<(), Error> {
    use HostStar::*;
    match &self {
      Star(star) => star.check_habitable()?,
      CloseBinaryStar(close_binary_star) => close_binary_star.check_habitable()?,
    }

    Ok(())
  }

  /// Indicate whether this star is capable of supporting conventional life.

  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }
}
