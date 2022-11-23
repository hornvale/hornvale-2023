use super::DGmCm3;
use crate::astronomy::_constants::*;

/// The `DSol` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct DSol(pub f64);

impl From<DGmCm3> for DSol {
  fn from(original: DGmCm3) -> Self {
    Self(original.0 / DENSITY_OF_SOL.0)
  }
}
