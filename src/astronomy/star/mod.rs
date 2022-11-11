use rand::prelude::*;

pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;
use error::*;
pub mod math;
use math::color::star_mass_to_rgb;
use math::frost_line::star_luminosity_to_frost_line;
use math::habitable_zone::star_luminosity_to_habitable_zone;
use math::luminosity::star_mass_to_luminosity;
use math::radius::star_mass_to_radius;
use math::satellite_zone::{get_approximate_innermost_orbit, get_approximate_outermost_orbit};
use math::spectral_class::star_mass_to_spectral_class;
use math::temperature::star_mass_to_temperature;
pub mod name;
use name::generate_star_name;

/// The `Star` type.
///
/// This is intended to encompass the most useful information we can generate
/// about main-sequence stars.  Other types will use different structs; it's
/// useful to view and treat these as the default sense of "star", given their
/// centrality to our purpose.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Star {
  /// Type, Decile, Luminosity class.
  pub class: String,
  /// Measured in Msol.
  pub mass: f64,
  /// Measured in Kelvin.
  pub temperature: f64,
  /// Measured in Rsol.
  pub radius: f64,
  /// Measured in Lsol.
  pub luminosity: f64,
  /// Measured in Gyr.
  pub life_expectancy: f64,
  /// Measured in Gyr.
  pub current_age: f64,
  /// Measured in Dsol.
  pub density: f64,
  /// Habitable zone, measured in AU.
  pub habitable_zone: (f64, f64),
  /// Minimum and maximum sustainable distance for satellites, measured in AU.
  /// This is inferior to computing the Roche limit and Hill sphere, but we
  /// don't have enough information for that yet.
  pub satellite_zone: (f64, f64),
  /// The frost line, measured in AU.
  pub frost_line: f64,
  /// The absolute color of this star in SRGB.
  pub absolute_rgb: (u8, u8, u8),
  /// A generated name for this star.
  pub name: String,
}

/// Implementation of Star.
impl Star {
  /// Generate a random main-sequence star from a given mass.
  pub fn from_mass<R: Rng + ?Sized>(rng: &mut R, mass: f64) -> Result<Star, Error> {
    let temperature = star_mass_to_temperature(mass)?;
    let luminosity = star_mass_to_luminosity(mass)?;
    let radius = star_mass_to_radius(mass)?;
    let class = star_mass_to_spectral_class(mass)?;
    let life_expectancy = mass / luminosity * 10.0;
    let lower_bound_age = 0.1 * life_expectancy;
    let upper_bound_age = 0.9 * life_expectancy;
    let current_age = rng.gen_range(lower_bound_age..upper_bound_age);
    let density = mass / radius.powf(3.0);
    let habitable_zone = star_luminosity_to_habitable_zone(luminosity);
    let satellite_inner_bound = get_approximate_innermost_orbit(mass);
    let satellite_outer_bound = get_approximate_outermost_orbit(mass);
    let satellite_zone = (satellite_inner_bound, satellite_outer_bound);
    let frost_line = star_luminosity_to_frost_line(luminosity);
    let absolute_rgb = star_mass_to_rgb(mass)?;
    let name = generate_star_name(rng);
    let result = Star {
      class,
      mass,
      luminosity,
      radius,
      temperature,
      life_expectancy,
      current_age,
      density,
      habitable_zone,
      satellite_zone,
      frost_line,
      absolute_rgb,
      name,
    };

    Ok(result)
  }

  /// Indicate whether this star is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
    if self.mass < MINIMUM_HABITABLE_MASS {
      return Err(Error::MassTooLowToSupportLife);
    }
    if self.mass > MAXIMUM_HABITABLE_MASS {
      return Err(Error::MassTooHighToSupportLife);
    }
    if self.current_age < MINIMUM_HABITABLE_AGE {
      return Err(Error::TooYoungToSupportLife);
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
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let star = Constraints::default().generate(&mut rng)?;

    print_var!(star);

    Ok(())
  }
}
