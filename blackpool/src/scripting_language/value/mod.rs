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
    test_instructions!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_instructions!([Negate], [Number(-53.0)] => [Number(53.0)]);
    // The order of the following binary operations can be a bit counterintuitive.
    // With a binary operation, this follows the pattern:
    //
    // test_instructions!([operation], [a, b] => [b operation a]);
    //
    // So for subtraction:
    //
    // test_instructions!([-], [a, b] => [b - a]);
    //
    // As a concrete example:
    //
    // test_instructions!([-], [10, 1] => [1 - 10 = -9]);
    test_instructions!([Add], [Number(-53.0), Number(4.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(4.0), Number(-53.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(-3.0), Number(4.0)] => [Number(1.0)]);
    test_instructions!([Add], [Number(4.0), Number(3.0)] => [Number(7.0)]);
    test_instructions!([Subtract], [Number(-53.0), Number(4.0)] => [Number(57.0)]);
    test_instructions!([Subtract], [Number(4.0), Number(-53.0)] => [Number(-57.0)]);
    test_instructions!([Subtract], [Number(-3.0), Number(4.0)] => [Number(7.0)]);
    test_instructions!([Subtract], [Number(4.0), Number(3.0)] => [Number(-1.0)]);
    test_instructions!([Multiply], [Number(4.0), Number(5.0)] => [Number(20.0)]);
    test_instructions!([Multiply], [Number(2.0), Number(-5.0)] => [Number(-10.0)]);
    test_instructions!([Divide], [Number(4.0), Number(5.0)] => [Number(1.25)]);
    test_instructions!([Divide], [Number(32.0), Number(-128.0)] => [Number(-4.0)]);
    test_instructions!([Add, Divide, Negate], [Number(1.2), Number(3.4), Number(5.6)] => [Number(-1.2173)]);
    trace_exit!();
  }
}
