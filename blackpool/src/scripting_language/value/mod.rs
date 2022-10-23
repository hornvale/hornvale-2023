/// The `Value` enum.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
  /// Number is a double.
  Number(f64),
}
