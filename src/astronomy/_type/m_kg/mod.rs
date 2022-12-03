use super::MSol;
use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;

/// The `MKg` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MKg(pub f64);

impl From<MSol> for MKg {
  fn from(original: MSol) -> Self {
    Self(original.0 * KG_PER_SOLAR_MASS.0)
  }
}

impl From<MJupiter> for MKg {
  fn from(original: MJupiter) -> Self {
    Self(original.0 * KG_PER_JUPITER_MASS.0)
  }
}

impl From<MEarth> for MKg {
  fn from(original: MEarth) -> Self {
    Self(original.0 * KG_PER_EARTH_MASS.0)
  }
}

impl From<MLuna> for MKg {
  fn from(original: MLuna) -> Self {
    Self(original.0 * KG_PER_LUNAR_MASS.0)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_m_sol_to_m_kg() {
    init();
    let actual: MKg = MSol(1.0).into();
    assert_approx_eq!(actual.0, KG_PER_SOLAR_MASS.0, 0.01);
  }

  #[test]
  pub fn test_m_earth_to_m_kg() {
    init();
    let actual: MKg = MEarth(1.0).into();
    assert_approx_eq!(actual.0, KG_PER_EARTH_MASS.0, 0.01);
  }
}
