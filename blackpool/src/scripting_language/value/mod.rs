use std::ops::{Add, Div, Mul, Neg, Sub};

/// The `Value` enum.
#[derive(Clone, Copy, Debug, Display, Deserialize, PartialEq, Serialize)]
pub enum Value {
  /// Number is a double.
  Number(f64),
}

impl Add for Value {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    use Value::*;
    match (self, other) {
      (Number(number), Number(other)) => Number(number + other),
    }
  }
}

impl Div for Value {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    use Value::*;
    match (self, other) {
      (Number(number), Number(other)) => Number(number / other),
    }
  }
}

impl Mul for Value {
  type Output = Self;
  fn mul(self, other: Self) -> Self::Output {
    use Value::*;
    match (self, other) {
      (Number(number), Number(other)) => Number(number * other),
    }
  }
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

impl Sub for Value {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    use Value::*;
    match (self, other) {
      (Number(number), Number(other)) => Number(number - other),
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_math() {
    init();
    trace_enter!();
    use crate::scripting_language::value::Value::*;
    test_program!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_program!([Negate], [Number(-53.0)] => [Number(53.0)]);
    // The order of the following binary operations can be a bit counterintuitive.
    // With a binary operation, this follows the pattern:
    //
    // test_program!([operation], [a, b] => [b operation a]);
    //
    // So for subtraction:
    //
    // test_program!([-], [a, b] => [b - a]);
    //
    // As a concrete example:
    //
    // test_program!([-], [10, 1] => [1 - 10 = -9]);
    test_program!([Add], [Number(-53.0), Number(4.0)] => [Number(-49.0)]);
    test_program!([Add], [Number(4.0), Number(-53.0)] => [Number(-49.0)]);
    test_program!([Add], [Number(-3.0), Number(4.0)] => [Number(1.0)]);
    test_program!([Add], [Number(4.0), Number(3.0)] => [Number(7.0)]);
    test_program!([Subtract], [Number(-53.0), Number(4.0)] => [Number(57.0)]);
    test_program!([Subtract], [Number(4.0), Number(-53.0)] => [Number(-57.0)]);
    test_program!([Subtract], [Number(-3.0), Number(4.0)] => [Number(7.0)]);
    test_program!([Subtract], [Number(4.0), Number(3.0)] => [Number(-1.0)]);
    trace_exit!();
  }
}
