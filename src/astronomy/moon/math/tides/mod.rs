use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;

/// Calculate the magnitude of the lunar tide.
/// `lunar_mass` - mass of the moon, in MLuna.
/// `planet_radius`  - radius of the planet, in Rearth.
/// `semi_major_axis` - semi-major axis of the moon's orbit, in KM.
///
/// Returns a magnitude in meters.
pub fn get_lunar_tide(lunar_mass: MLuna, planet_radius: REarth, semi_major_axis: LKm) -> Lm {
  let corrected_lunar_mass = 2_230_000.0 * lunar_mass.0 * LUNA_GRAVITATIONAL_PARAMETER_SHARE;

  Lm((corrected_lunar_mass * planet_radius.0) / (semi_major_axis.0 / KM_PER_EARTH_DIAMETER.0).powf(3.0))
}

/// Calculate the magnitude of the solar tide.
/// `star_mass` - mass of the sun, in Msun.
/// `planet_radius`  - radius of the planet, in Rearth.
/// `semi_major_axis` - semi-major axis of the planet's orbit, in AU.
///
/// Returns a magnitude in meters.
pub fn get_solar_tide(star_mass: MSol, planet_radius: REarth, semi_major_axis: LAu) -> Lm {
  Lm((0.46 * star_mass.0 * planet_radius.0) / semi_major_axis.0.powf(3.0))
}

/// Calculate the magnitude of the planetary tide.
/// `moon_mass` - mass of the moon, in MLuna.
/// `moon_radius`  - radius of the moon, in RLuna.
/// `semi_major_axis` - semi-major axis of the moon's orbit, in KM.
///
/// Returns a magnitude in meters.
pub fn get_planetary_tide(moon_mass: MLuna, moon_radius: RLuna, semi_major_axis: LKm) -> Lm {
  Lm((2_230_000.0 * moon_mass.0 * moon_radius.0 * 0.027264) / (semi_major_axis.0 / KM_PER_EARTH_DIAMETER.0).powf(3.0))
}

/// Calculate the magnitude of the spring tides.
pub fn get_spring_tide(lunar_tide: Lm, solar_tide: Lm) -> Lm {
  Lm(((lunar_tide.0 + solar_tide.0) * 0.54).abs())
}

/// Calculate the magnitude of the neap tides.
pub fn get_neap_tide(lunar_tide: Lm, solar_tide: Lm) -> Lm {
  Lm(((lunar_tide.0 - solar_tide.0) * 0.54).abs())
}

/// Determine whether the planet is tidally locked.
pub fn is_planet_tidally_locked(lunar_tide: Lm, solar_tide: Lm, star_age: TGyr, planet_mass: MEarth) -> bool {
  ((lunar_tide.0 + solar_tide.0) * star_age.0 / planet_mass.0) > 50.0
}

/// Determine whether the moon is tidally locked.
pub fn is_moon_tidally_locked(solar_tide: Lm, planet_tide: Lm, star_age: TGyr, moon_mass: MLuna) -> bool {
  (((solar_tide.0 + planet_tide.0) * star_age.0) / (moon_mass.0 * LUNA_GRAVITATIONAL_PARAMETER_SHARE)) > 50.0
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_get_lunar_tide() {
    init();
    let actual = get_lunar_tide(MLuna(1.0), REarth(1.0), LKm(384_784.0));
    print_var!(actual);
    let expected = 0.996;
    assert_approx_eq!(expected, actual.0, 0.001);
  }

  #[test]
  pub fn test_get_solar_tide() {
    init();
    let actual = get_solar_tide(MSol(1.0), REarth(1.0), LAu(1.0));
    print_var!(actual);
    let expected = 0.460;
    assert_approx_eq!(expected, actual.0, 0.001);
  }

  #[test]
  pub fn test_get_planetary_tide() {
    init();
    let actual = get_planetary_tide(MLuna(1.0), RLuna(1.0), LKm(384_784.0));
    print_var!(actual);
    let expected = 2.2077;
    assert_approx_eq!(expected, actual.0, 0.001);
  }

  #[test]
  pub fn test_get_spring_tide() {
    init();
    let actual = get_spring_tide(Lm(0.996), Lm(0.460));
    print_var!(actual);
    let expected = 0.78624;
    assert_approx_eq!(expected, actual.0, 0.001);
  }

  #[test]
  pub fn test_get_neap_tide() {
    init();
    let actual = get_neap_tide(Lm(0.996), Lm(0.460));
    print_var!(actual);
    let expected = 0.28944;
    assert_approx_eq!(expected, actual.0, 0.001);
  }
}
