use super::DGmCm3;
use crate::astronomy::_constant::*;

/// The `DEarth` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct DEarth(pub f64);

impl From<DGmCm3> for DEarth {
  fn from(original: DGmCm3) -> Self {
    Self(original.0 / DENSITY_OF_EARTH.0)
  }
}
