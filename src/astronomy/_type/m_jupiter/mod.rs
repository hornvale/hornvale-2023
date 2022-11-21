use super::MEarth;
use crate::astronomy::_constants::*;

/// The `MJupiter` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MJupiter(pub f64);

impl From<MEarth> for MJupiter {
  fn from(original: MEarth) -> Self {
    Self(original.0 / EARTH_MASS_PER_JUPITER_MASS)
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
