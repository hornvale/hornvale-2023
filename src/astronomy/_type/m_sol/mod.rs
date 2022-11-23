use crate::astronomy::MEarth;
use crate::astronomy::MJupiter;
use crate::astronomy::_constants::*;

/// The `MSol` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MSol(pub f64);

impl From<MJupiter> for MSol {
  fn from(original: MJupiter) -> Self {
    Self(original.0 / JUPITER_MASS_PER_SOLAR_MASS.0)
  }
}

impl From<MEarth> for MSol {
  fn from(original: MEarth) -> Self {
    Self(original.0 / EARTH_MASS_PER_SOLAR_MASS.0)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_m_earth_to_m_sol() {
    init();
    let actual: MSol = MEarth(1.0).into();
    assert_approx_eq!(actual.0, EARTH_MASS_PER_SOLAR_MASS, 0.01);
  }

  #[test]
  pub fn test_m_kg_to_m_sol() {
    init();
    let actual: MSol = MKg(1.0).into();
    assert_approx_eq!(actual.0, 1.0 / KG_PER_SOLAR_MASS, 0.01);
  }
}
