use rand::prelude::*;

use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star::Star;

pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;
use math::barycenter::get_average_distances_from_barycenter;
use math::barycenter::get_maximum_distances_from_barycenter;
use math::barycenter::get_minimum_distances_from_barycenter;
use math::frost_line::get_frost_line;
use math::habitable_zone::get_habitable_zone;
use math::separation::get_maximum_separation;
use math::separation::get_minimum_separation;

/// A `CloseBinaryStar` is a system of two `Star` objects.
///
/// This may seem counterintuitive, but a `CloseBinaryStar` is actually more
/// closely related to a `Star` than a `DistantBinaryStar`.  The reason for
/// this is that habitable planets can be in a circumbinary orbit around a
/// `CloseBinaryStar`, but can only be in an orbit around one member of a
/// `DistantBinaryStar`.  As a result, we handle `DistantBinaryStar` objects
/// with a different class.
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBinaryStar {
  /// The primary star is the one with greater mass.
  pub primary: Star,
  /// The secondary star has less mass.
  pub secondary: Star,
  /// Average separation of the binary components, in AU.
  pub average_separation: f64,
  /// Orbital eccentricity of the components.
  pub orbital_eccentricity: f64,
  /// Average distance from barycenter of the components.
  pub average_distances_from_barycenter: (f64, f64),
  /// Minimum distance from barycenter of the components.
  pub minimum_distances_from_barycenter: (f64, f64),
  /// Maximum distance from barycenter of the components.
  pub maximum_distances_from_barycenter: (f64, f64),
  /// Minimum separation of the components, in AU.
  pub minimum_separation: f64,
  /// Maximum separation of the components, in AU.
  pub maximum_separation: f64,
  /// Area in which nothing can exist.
  pub forbidden_zone: (f64, f64),
  /// Area in which nothing _habitable_ can exist.
  pub danger_zone: (f64, f64),
  /// Habitable zone.
  pub habitable_zone: (f64, f64),
  /// Satellite bounds.
  pub satellite_zone: (f64, f64),
  /// The frost line.
  pub frost_line: f64,
  /// Whether the habitable zone is contained within the forbidden zone.
  pub habitable_zone_is_forbidden: bool,
  /// Whether the habitable zone is contained within the danger zone.
  pub habitable_zone_is_dangerous: bool,
}

impl CloseBinaryStar {
  /// Create from a pair of stars, average separation, and orbital eccentricity.
  #[named]
  pub fn from_stars<R: Rng + ?Sized>(
    _rng: &mut R,
    primary: Star,
    secondary: Star,
    average_separation: f64,
    orbital_eccentricity: f64,
  ) -> Result<Self, Error> {
    trace_enter!();
    trace_var!(primary);
    trace_var!(secondary);
    trace_var!(average_separation);
    trace_var!(orbital_eccentricity);
    let average_distances_from_barycenter =
      get_average_distances_from_barycenter(average_separation, primary.mass, secondary.mass);
    trace_var!(average_distances_from_barycenter);
    let minimum_distances_from_barycenter =
      get_minimum_distances_from_barycenter(average_separation, primary.mass, secondary.mass, orbital_eccentricity);
    trace_var!(minimum_distances_from_barycenter);
    let minimum_separation = get_minimum_separation(minimum_distances_from_barycenter);
    trace_var!(minimum_separation);
    if minimum_separation < MINIMUM_SEPARATION {
      return Err(Error::BinaryStarsTooCloseForComfort);
    }
    let maximum_distances_from_barycenter =
      get_maximum_distances_from_barycenter(average_separation, primary.mass, secondary.mass, orbital_eccentricity);
    trace_var!(maximum_distances_from_barycenter);
    let maximum_separation = get_maximum_separation(maximum_distances_from_barycenter);
    trace_var!(maximum_separation);
    let forbidden_zone = (minimum_separation / 3.0, maximum_separation * 3.0);
    trace_var!(forbidden_zone);
    let danger_zone = (0.0, maximum_separation * 4.0);
    trace_var!(danger_zone);
    let habitable_zone = get_habitable_zone(&primary, &secondary);
    trace_var!(habitable_zone);
    let combined_mass = primary.mass + secondary.mass;
    let satellite_zone = (0.1 * combined_mass, 40.0 * combined_mass);
    trace_var!(satellite_zone);
    let frost_line = get_frost_line(&primary, &secondary);
    trace_var!(frost_line);
    let habitable_zone_is_forbidden = habitable_zone.1 <= forbidden_zone.1;
    trace_var!(habitable_zone_is_forbidden);
    let habitable_zone_is_dangerous = habitable_zone.1 <= danger_zone.1;
    trace_var!(habitable_zone_is_dangerous);
    let result = CloseBinaryStar {
      primary,
      secondary,
      average_separation,
      orbital_eccentricity,
      average_distances_from_barycenter,
      minimum_distances_from_barycenter,
      maximum_distances_from_barycenter,
      minimum_separation,
      maximum_separation,
      forbidden_zone,
      danger_zone,
      habitable_zone,
      satellite_zone,
      frost_line,
      habitable_zone_is_forbidden,
      habitable_zone_is_dangerous,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Retrieve or calculate the age of the stars.
  ///
  /// Calculated in Gyr.
  #[named]
  pub fn get_current_age(&self) -> f64 {
    trace_enter!();
    let result = self.primary.current_age;
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
    let result = self.primary.mass + self.secondary.mass;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Measured in Lsol.
  #[named]
  pub fn get_luminosity(&self) -> f64 {
    trace_enter!();
    let result = self.primary.luminosity + self.secondary.luminosity;
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  pub fn get_name(&self) -> String {
    trace_enter!();
    let result = format!("{}-{}", self.primary.name, self.secondary.name);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this StarSubsystem is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    if self.habitable_zone_is_forbidden {
      return Err(Error::HabitableZoneContainedWithinForbiddenZone);
    }
    if self.habitable_zone_is_dangerous {
      return Err(Error::HabitableZoneContainedWithinDangerZone);
    }
    self.primary.check_habitable()?;
    // Secondary stars can be very low mass or young but still habitable.
    match self.secondary.check_habitable() {
      Err(StarError::MassTooLowToSupportLife) => {},
      Ok(_) => {},
      Err(error) => return Err(error.into()),
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

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_whatever() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = &Constraints::default().generate(&mut rng)?;
    trace_var!(star);
    print_var!(star);
    trace_exit!();
    Ok(())
  }
}
