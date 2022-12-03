use super::LKm;
use crate::astronomy::_constant::*;

/// The `REarth` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct REarth(pub f64);

impl From<LKm> for REarth {
  fn from(original: LKm) -> Self {
    Self(original.0 / KM_PER_EARTH_RADIUS.0)
  }
}
