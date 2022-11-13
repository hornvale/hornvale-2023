pub const EARTH_MASS_PER_JUPITER_MASS: f64 = 317.8;

/// Convert from Mearth to Mjupiter.
pub fn earth_mass_to_jupiter_mass(mass: f64) -> f64 {
  mass / EARTH_MASS_PER_JUPITER_MASS
}

/// Convert from Mjupiter to Mearth.
pub fn jupiter_mass_to_earth_mass(mass: f64) -> f64 {
  mass * EARTH_MASS_PER_JUPITER_MASS
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_msol_to_kg() {
    init();
    assert_approx_eq!(jupiter_mass_to_earth_mass(earth_mass_to_jupiter_mass(1.0)), 1.0);
    assert_approx_eq!(jupiter_mass_to_earth_mass(1.0), EARTH_MASS_PER_JUPITER_MASS);
  }
}
