use crate::astronomy::_type::*;
use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;

/// Get the radius of a main-sequence star in Rsol based on its Msol.
pub fn star_mass_to_radius(mass: MSol) -> Result<RSol, Error> {
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass.0 < 1.0 => mass.0.powf(0.80),
    mass if mass.0 >= 1.0 => mass.0.powf(0.57),
    _ => unreachable!(),
  };

  Ok(RSol(result))
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_ms_star_mass_to_radius() -> Result<(), Error> {
    init();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 1.0;
    let mut actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual);
    // M1V
    mass = 0.40;
    expected = 0.480;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // K9V
    mass = 0.50;
    expected = 0.574;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // G7V
    mass = 0.90;
    expected = 0.919;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // F6V
    mass = 1.20;
    expected = 1.110;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 1.353;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // B5V
    mass = 8.0;
    expected = 3.272;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 6.264;
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    Ok(())
  }
}
