use super::REarth;
use super::RJupiter;
use crate::astronomy::_constants::*;

/// The `LKm` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct LKm(pub f64);

impl From<REarth> for LKm {
  fn from(original: REarth) -> Self {
    Self(original.0 * KM_PER_EARTH_RADIUS.0)
  }
}

impl From<RJupiter> for LKm {
  fn from(original: RJupiter) -> Self {
    Self(original.0 * KM_PER_JUPITER_RADIUS.0)
  }
}
