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
  #[named]
  pub fn get_current_age(&self) -> f64 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.current_age,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_current_age(),
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
    use HostStar::*;
    let result = match &self {
      Star(star) => star.mass,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_stellar_mass(),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(_) => 1,
      CloseBinaryStar(_) => 2,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the frost line.
  #[named]
  pub fn get_frost_line(&self) -> f64 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.frost_line,
      CloseBinaryStar(close_binary_star) => close_binary_star.frost_line,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the habitable zone.
  #[named]
  pub fn get_habitable_zone(&self) -> (f64, f64) {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.habitable_zone,
      CloseBinaryStar(close_binary_star) => close_binary_star.habitable_zone,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the satellite zone.
  #[named]
  pub fn get_satellite_zone(&self) -> (f64, f64) {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.satellite_zone,
      CloseBinaryStar(close_binary_star) => close_binary_star.satellite_zone,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the luminosity.
  #[named]
  pub fn get_luminosity(&self) -> f64 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.luminosity,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_luminosity(),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    use HostStar::*;
    match &self {
      Star(star) => star.check_habitable()?,
      CloseBinaryStar(close_binary_star) => close_binary_star.check_habitable()?,
    }
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
}
