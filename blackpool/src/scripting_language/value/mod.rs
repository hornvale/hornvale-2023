use std::ops::Neg;

/// The `Value` enum.
#[derive(Clone, Copy, Debug, Display, Deserialize, PartialEq, Serialize)]
pub enum Value {
  /// Number is a double.
  Number(f64),
}

impl Neg for Value {
  type Output = Self;
  fn neg(self) -> Self::Output {
    use Value::*;
    match self {
      Number(number) => Number(-number),
    }
  }
}
