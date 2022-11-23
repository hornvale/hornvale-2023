use super::MEarth;
use super::MKg;
use crate::astronomy::_constants::*;
use crate::astronomy::_type::*;

/// The `MJupiter` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MJupiter(pub f64);

impl From<MEarth> for MJupiter {
  fn from(original: MEarth) -> Self {
    Self(original.0 / EARTH_MASS_PER_JUPITER_MASS.0)
  }
}

impl From<MSol> for MJupiter {
  fn from(original: MSol) -> Self {
    Self(original.0 * JUPITER_MASS_PER_SOLAR_MASS.0)
  }
}

impl From<MKg> for MJupiter {
  fn from(original: MKg) -> Self {
    Self(original.0 / KG_PER_JUPITER_MASS.0)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_m_earth_to_m_jupiter() {
    init();
    let actual: MJupiter = MEarth(1.0).into();
    assert_approx_eq!(actual.0, MJupiter(1.0 / EARTH_MASS_PER_JUPITER_MASS).0, 0.01);
  }
}
