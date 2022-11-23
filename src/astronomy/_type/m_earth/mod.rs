use super::MJupiter;
use super::MKg;
use super::MLuna;
use crate::astronomy::_constants::*;

/// The `MEarth` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MEarth(pub f64);

impl From<MJupiter> for MEarth {
  fn from(original: MJupiter) -> Self {
    Self(EARTH_MASS_PER_JUPITER_MASS.0 * original.0)
  }
}

impl From<MLuna> for MEarth {
  fn from(original: MLuna) -> Self {
    Self(original.0 / LUNA_MASS_PER_EARTH_MASS.0)
  }
}

impl From<MKg> for MEarth {
  fn from(original: MKg) -> Self {
    Self(original.0 / KG_PER_EARTH_MASS.0)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_m_jupiter_to_m_earth() {
    init();
    let actual: MEarth = MJupiter(1.0).into();
    assert_approx_eq!(actual.0, EARTH_MASS_PER_JUPITER_MASS.0, 0.01);
  }

  #[test]
  pub fn test_m_luna_to_m_earth() {
    init();
    let actual: MEarth = MLuna(1.0).into();
    assert_approx_eq!(actual.0, 1.0 / LUNA_MASS_PER_EARTH_MASS.0, 0.01);
  }

  #[test]
  pub fn test_m_kg_to_m_earth() {
    init();
    let actual: MEarth = MKg(1.0).into();
    assert_approx_eq!(actual.0, 1.0 / KG_PER_EARTH_MASS.0, 0.01);
  }
}
