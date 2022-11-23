use super::DEarth;
use super::DJupiter;
use super::DLuna;
use super::DSol;
use crate::astronomy::_constants::*;

/// The `DGmCm3` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct DGmCm3(pub f64);

impl From<DJupiter> for DGmCm3 {
  fn from(original: DJupiter) -> Self {
    Self(DENSITY_OF_JUPITER.0 * original.0)
  }
}

impl From<DEarth> for DGmCm3 {
  fn from(original: DEarth) -> Self {
    Self(DENSITY_OF_EARTH.0 * original.0)
  }
}

impl From<DLuna> for DGmCm3 {
  fn from(original: DLuna) -> Self {
    Self(DENSITY_OF_LUNA.0 * original.0)
  }
}

impl From<DSol> for DGmCm3 {
  fn from(original: DSol) -> Self {
    Self(DENSITY_OF_SOL.0 * original.0)
  }
}
