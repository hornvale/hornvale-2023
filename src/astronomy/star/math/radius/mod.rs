use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;

/// Get the radius of a main-sequence star in Rsol based on its Msol.
#[named]
pub fn star_mass_to_radius(mass: f64) -> Result<f64, Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass < 1.0 => mass.powf(0.80),
    mass if mass >= 1.0 => mass.powf(0.57),
    _ => unreachable!(),
  };
  trace_var!(result);
  trace_exit!();
  Ok(result)
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_ms_star_mass_to_radius() -> Result<(), Error> {
    init();
    trace_enter!();
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
    trace_exit!();
    Ok(())
  }
}
