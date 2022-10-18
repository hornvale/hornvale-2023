use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;

pub const ERGS_PER_SEC_PER_LSOL: f64 = 3.846E33;
pub const JOULES_PER_SEC_PER_LSOL: f64 = 3.846E26;

/// ergs/sec -> Lsol
pub fn ergs_to_lsol(ergs: f64) -> f64 {
  ergs / ERGS_PER_SEC_PER_LSOL
}

/// Lsol -> ergs/sec
pub fn lsol_to_ergs(lsol: f64) -> f64 {
  lsol * ERGS_PER_SEC_PER_LSOL
}

/// J/sec -> Lsol
pub fn joules_to_lsol(joules: f64) -> f64 {
  joules / JOULES_PER_SEC_PER_LSOL
}

/// Lsol -> J/sec
pub fn lsol_to_joules(lsol: f64) -> f64 {
  lsol * JOULES_PER_SEC_PER_LSOL
}

/// W -> Lsol
pub fn watts_to_lsol(watts: f64) -> f64 {
  watts / JOULES_PER_SEC_PER_LSOL
}

/// Lsol -> W
pub fn lsol_to_watts(lsol: f64) -> f64 {
  lsol * JOULES_PER_SEC_PER_LSOL
}

/// Get the luminosity of a main-sequence star in Lsol based on its Msol.
#[named]
pub fn star_mass_to_luminosity(mass: f64) -> Result<f64, Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass < 0.43 => 0.23 * mass.powf(2.3),
    mass if mass < 2.0 => mass.powf(4.0),
    mass if mass < 55.0 => 1.4 * mass.powf(3.5),
    mass if mass < MAXIMUM_MASS => 32_000.0 * mass,
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
  pub fn test_ms_star_mass_to_luminosity() -> Result<(), Error> {
    init();
    trace_enter!();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 1.0;
    let mut actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual);
    // M1V
    mass = 0.40;
    expected = 0.028;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // K9V
    mass = 0.50;
    expected = 0.063;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // G7V
    mass = 0.90;
    expected = 0.656;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // F6V
    mass = 1.20;
    expected = 2.073;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 8.352;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 8.352;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // B5V
    mass = 8.0;
    expected = 2027.4;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 109375.0;
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    trace_exit!();
    Ok(())
  }
}
