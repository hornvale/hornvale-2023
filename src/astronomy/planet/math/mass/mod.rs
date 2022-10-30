pub const EARTH_MASS_PER_JUPITER_MASS: f64 = 317.8;

/// Convert from Mearth to Mjupiter.
#[named]
pub fn earth_mass_to_jupiter_mass(mass: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  let result = mass / EARTH_MASS_PER_JUPITER_MASS;
  trace_var!(result);
  trace_exit!();
  result
}

/// Convert from Mjupiter to Mearth.
#[named]
pub fn jupiter_mass_to_earth_mass(mass: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  let result = mass * EARTH_MASS_PER_JUPITER_MASS;
  trace_var!(result);
  trace_exit!();
  result
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_msol_to_kg() {
    init();
    trace_enter!();
    assert_approx_eq!(jupiter_mass_to_earth_mass(earth_mass_to_jupiter_mass(1.0)), 1.0);
    assert_approx_eq!(jupiter_mass_to_earth_mass(1.0), EARTH_MASS_PER_JUPITER_MASS);
    trace_exit!();
  }
}
