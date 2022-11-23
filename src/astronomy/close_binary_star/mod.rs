use crate::astronomy::_type::*;
use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star::Star;
use rand::prelude::*;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CloseBinaryStar {
  /// The primary star is the one with greater mass.
  pub primary: Star,
  /// The secondary star has less mass.
  pub secondary: Star,
  /// Average separation of the binary components, in AU.
  pub average_separation: LAu,
  /// Orbital eccentricity of the components (unitless).
  pub orbital_eccentricity: f64,
  /// Average distance from barycenter of the components.
  pub average_distances_from_barycenter: (LAu, LAu),
  /// Minimum distance from barycenter of the components.
  pub minimum_distances_from_barycenter: (LAu, LAu),
  /// Maximum distance from barycenter of the components.
  pub maximum_distances_from_barycenter: (LAu, LAu),
  /// Minimum separation of the components, in AU.
  pub minimum_separation: LAu,
  /// Maximum separation of the components, in AU.
  pub maximum_separation: LAu,
  /// Area in which nothing can exist.
  pub forbidden_zone: (LAu, LAu),
  /// Area in which nothing _habitable_ can exist.
  pub danger_zone: (LAu, LAu),
  /// Habitable zone.
  pub habitable_zone: (LAu, LAu),
  /// Satellite bounds.
  pub satellite_zone: (LAu, LAu),
  /// The frost line.
  pub frost_line: LAu,
  /// Whether the habitable zone is contained within the forbidden zone.
  pub habitable_zone_is_forbidden: bool,
  /// Whether the habitable zone is contained within the danger zone.
  pub habitable_zone_is_dangerous: bool,
}

impl CloseBinaryStar {
  /// Create from a pair of stars, average separation, and orbital eccentricity.
  pub fn from_stars<R: Rng + ?Sized>(
    _rng: &mut R,
    primary: Star,
    secondary: Star,
    average_separation: LAu,
    orbital_eccentricity: f64,
  ) -> Result<Self, Error> {
    let average_distances_from_barycenter =
      get_average_distances_from_barycenter(average_separation, primary.mass, secondary.mass);
    let minimum_distances_from_barycenter =
      get_minimum_distances_from_barycenter(average_separation, primary.mass, secondary.mass, orbital_eccentricity);
    let minimum_separation = get_minimum_separation(minimum_distances_from_barycenter);
    if minimum_separation < MINIMUM_SEPARATION {
      return Err(Error::BinaryStarsTooCloseForComfort);
    }
    let maximum_distances_from_barycenter =
      get_maximum_distances_from_barycenter(average_separation, primary.mass, secondary.mass, orbital_eccentricity);
    let maximum_separation = get_maximum_separation(maximum_distances_from_barycenter);
    let forbidden_zone = (minimum_separation / 3.0, maximum_separation * 3.0);
    let danger_zone = (LAu(0.0), LAu(maximum_separation.0 * 4.0));
    let habitable_zone = get_habitable_zone(&primary, &secondary);
    let combined_mass = primary.mass + secondary.mass;
    let satellite_zone = (LAu(0.1 * combined_mass.0), LAu(40.0 * combined_mass.0));
    let frost_line = get_frost_line(&primary, &secondary);
    let habitable_zone_is_forbidden = habitable_zone.1 <= forbidden_zone.1;
    let habitable_zone_is_dangerous = habitable_zone.1 <= danger_zone.1;
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
    Ok(result)
  }

  /// Retrieve or calculate the age of the stars.
  ///
  /// Calculated in Gyr.
  pub fn get_current_age(&self) -> TGyr {
    self.primary.current_age
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  pub fn get_stellar_mass(&self) -> MSol {
    self.primary.mass + self.secondary.mass
  }

  /// Measured in Lsol.
  pub fn get_luminosity(&self) -> LSol {
    self.primary.luminosity + self.secondary.luminosity
  }

  pub fn get_name(&self) -> String {
    let result = format!("{}-{}", self.primary.name, self.secondary.name);
    result
  }

  /// Indicate whether this StarSubsystem is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
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

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_whatever() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let star = &Constraints::default().generate(&mut rng)?;
    print_var!(star);
    Ok(())
  }
}
