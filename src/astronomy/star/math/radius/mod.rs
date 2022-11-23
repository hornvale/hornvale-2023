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
    let mut mass = MSol(1.0);
    let mut expected = RSol(1.0);
    let mut actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0);
    // M1V
    mass = MSol(0.40);
    expected = RSol(0.480);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // K9V
    mass = MSol(0.50);
    expected = RSol(0.574);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // G7V
    mass = MSol(0.90);
    expected = RSol(0.919);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // F6V
    mass = MSol(1.20);
    expected = RSol(1.110);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // A6V
    mass = MSol(1.70);
    expected = RSol(1.353);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // B5V
    mass = MSol(8.0);
    expected = RSol(3.272);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1f64);
    // O8V
    mass = MSol(25.0);
    expected = RSol(6.264);
    actual = star_mass_to_radius(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1f64);
    Ok(())
  }
}
