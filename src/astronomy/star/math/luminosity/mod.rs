use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;
use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;

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
pub fn star_mass_to_luminosity(mass: MSol) -> Result<LSol, Error> {
  if mass < MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass > MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass.0 < 0.43 => 0.23 * mass.0.powf(2.3),
    mass if mass.0 < 2.0 => mass.0.powf(4.0),
    mass if mass.0 < 55.0 => 1.4 * mass.0.powf(3.5),
    mass if mass <= MAXIMUM_MASS => 32_000.0 * mass.0,
    _ => unreachable!(),
  };

  Ok(LSol(result))
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_ergs_to_lsol() {
    init();
    assert_approx_eq!(ergs_to_lsol(lsol_to_ergs(1.0)), 1.0);
    assert_approx_eq!(lsol_to_ergs(1.0), ERGS_PER_SEC_PER_LSOL);
  }

  #[test]
  pub fn test_joules_to_lsol() {
    init();
    assert_approx_eq!(joules_to_lsol(lsol_to_joules(1.0)), 1.0);
    assert_approx_eq!(lsol_to_joules(1.0), JOULES_PER_SEC_PER_LSOL);
  }

  #[test]
  pub fn test_watts_to_lsol() {
    init();
    assert_approx_eq!(watts_to_lsol(lsol_to_watts(1.0)), 1.0);
    assert_approx_eq!(lsol_to_watts(1.0), JOULES_PER_SEC_PER_LSOL);
  }

  #[test]
  #[should_panic]
  pub fn test_star_mass_to_luminosity1() {
    init();
    star_mass_to_luminosity(MSol(0.0000001)).unwrap();
  }

  #[test]
  #[should_panic]
  pub fn test_star_mass_to_luminosity2() {
    init();
    star_mass_to_luminosity(MSol(350.0)).unwrap();
  }

  #[test]
  pub fn test_star_mass_to_luminosity3() {
    init();
    star_mass_to_luminosity(MAXIMUM_MASS).unwrap();
  }

  #[test]
  pub fn test_ms_star_mass_to_luminosity() -> Result<(), Error> {
    init();
    // Jolly ol' Sol
    let mut mass = MSol(1.0);
    let mut expected = LSol(1.0);
    let mut actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0);
    // M1V
    mass = MSol(0.40);
    expected = LSol(0.028);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // K9V
    mass = MSol(0.50);
    expected = LSol(0.063);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // G7V
    mass = MSol(0.90);
    expected = LSol(0.656);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // F6V
    mass = MSol(1.20);
    expected = LSol(2.073);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // A6V
    mass = MSol(1.70);
    expected = LSol(8.352);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // A6V
    mass = MSol(1.70);
    expected = LSol(8.352);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1e-3f64);
    // B5V
    mass = MSol(8.0);
    expected = LSol(2027.4);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1f64);
    // O8V
    mass = MSol(25.0);
    expected = LSol(109375.0);
    actual = star_mass_to_luminosity(mass)?;
    assert_approx_eq!(expected.0, actual.0, 1f64);
    Ok(())
  }
}
