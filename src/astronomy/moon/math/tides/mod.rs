/// Calculate the magnitude of the lunar tide.
/// `lunar_mass` - mass of the moon, in Mmoon.
/// `planet_radius`  - radius of the planet, in Rearth.
/// `semi_major_axis` - semi-major axis of the moon's orbit, in KM.
///
/// Returns a magnitude in meters.

pub fn get_lunar_tide(lunar_mass: f64, planet_radius: f64, semi_major_axis: f64) -> f64 {
  let corrected_lunar_mass = 2_230_000.0 * lunar_mass * 0.0123;

  (corrected_lunar_mass * planet_radius) / (semi_major_axis / 12_742.0).powf(3.0)
}

/// Calculate the magnitude of the solar tide.
/// `star_mass` - mass of the sun, in Msun.
/// `planet_radius`  - radius of the planet, in Rearth.
/// `semi_major_axis` - semi-major axis of the planet's orbit, in AU.
///
/// Returns a magnitude in meters.

pub fn get_solar_tide(star_mass: f64, planet_radius: f64, semi_major_axis: f64) -> f64 {
  (0.46 * star_mass * planet_radius) / semi_major_axis.powf(3.0)
}

/// Calculate the magnitude of the planetary tide.
/// `moon_mass` - mass of the moon, in Mmoon.
/// `moon_radius`  - radius of the moon, in Rmoon.
/// `semi_major_axis` - semi-major axis of the moon's orbit, in KM.
///
/// Returns a magnitude in meters.

pub fn get_planetary_tide(moon_mass: f64, moon_radius: f64, semi_major_axis: f64) -> f64 {
  (2_230_000.0 * moon_mass * moon_radius * 0.027264) / (semi_major_axis / 12_742.0).powf(3.0)
}

/// Calculate the magnitude of the spring tides.

pub fn get_spring_tide(lunar_tide: f64, solar_tide: f64) -> f64 {
  ((lunar_tide + solar_tide) * 0.54).abs()
}

/// Calculate the magnitude of the neap tides.

pub fn get_neap_tide(lunar_tide: f64, solar_tide: f64) -> f64 {
  ((lunar_tide - solar_tide) * 0.54).abs()
}

/// Determine whether the planet is tidally locked.

pub fn is_planet_tidally_locked(lunar_tide: f64, solar_tide: f64, star_age: f64, planet_mass: f64) -> bool {
  ((lunar_tide + solar_tide) * star_age / planet_mass) > 50.0
}

/// Determine whether the moon is tidally locked.

pub fn is_moon_tidally_locked(solar_tide: f64, planet_tide: f64, star_age: f64, moon_mass: f64) -> bool {
  (((solar_tide + planet_tide) * star_age) / (moon_mass * 0.0123)) > 50.0
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_get_lunar_tide() {
    init();
    let actual = get_lunar_tide(1.0, 1.0, 384_784.0);

    print_var!(actual);
    let expected = 0.996;
    assert_approx_eq!(expected, actual, 0.001);
  }

  #[test]
  pub fn test_get_solar_tide() {
    init();
    let actual = get_solar_tide(1.0, 1.0, 1.0);

    print_var!(actual);
    let expected = 0.460;
    assert_approx_eq!(expected, actual, 0.001);
  }

  #[test]
  pub fn test_get_planetary_tide() {
    init();
    let actual = get_planetary_tide(1.0, 1.0, 384_784.0);

    print_var!(actual);
    let expected = 2.2077;
    assert_approx_eq!(expected, actual, 0.001);
  }

  #[test]
  pub fn test_get_spring_tide() {
    init();
    let actual = get_spring_tide(0.996, 0.460);

    print_var!(actual);
    let expected = 0.78624;
    assert_approx_eq!(expected, actual, 0.001);
  }

  #[test]
  pub fn test_get_neap_tide() {
    init();
    let actual = get_neap_tide(0.996, 0.460);

    print_var!(actual);
    let expected = 0.28944;
    assert_approx_eq!(expected, actual, 0.001);
  }
}
