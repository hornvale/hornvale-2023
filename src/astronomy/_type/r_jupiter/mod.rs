use super::LKm;
use crate::astronomy::_constant::*;

/// The `RJupiter` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct RJupiter(pub f64);

impl From<LKm> for RJupiter {
  fn from(original: LKm) -> Self {
    Self(original.0 / KM_PER_JUPITER_RADIUS.0)
  }
}
