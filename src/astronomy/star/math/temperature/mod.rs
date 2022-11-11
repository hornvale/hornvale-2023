use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::luminosity::star_mass_to_luminosity;
use crate::astronomy::star::math::radius::star_mass_to_radius;

/// Get the temperature of a main-sequence star in Kelvin based on its Msol.
pub fn star_mass_to_temperature(mass: f64) -> Result<f64, Error> {
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let luminosity = star_mass_to_luminosity(mass)?;
  let radius = star_mass_to_radius(mass)?;
  let result = (luminosity / radius.powf(2.0)).powf(0.25) * 5776.0;

  Ok(result)
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_ms_star_mass_to_temperature() -> Result<(), Error> {
    init();

    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 5776.0;
    let mut actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // M1V
    mass = 0.40;
    expected = 3407.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // K9V
    mass = 0.50;
    expected = 3811.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // G7V
    mass = 0.90;
    expected = 5422.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // F6V
    mass = 1.20;
    expected = 6580.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // A6V
    mass = 1.70;
    expected = 8441.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // B5V
    mass = 8.0;
    expected = 21428.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 41970.0;
    actual = star_mass_to_temperature(mass)?;
    assert_approx_eq!(expected, actual, 1f64);

    Ok(())
  }
}
