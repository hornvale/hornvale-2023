use super::DGmCm3;
use crate::astronomy::_constants::*;

/// The `DJupiter` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct DJupiter(pub f64);

impl From<DGmCm3> for DJupiter {
  fn from(original: DGmCm3) -> Self {
    Self(original.0 / DENSITY_OF_JUPITER.0)
  }
}
