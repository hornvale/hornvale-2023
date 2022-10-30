/// Direction of rotation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RotationDirection {
  /// Forwards.
  Prograde,
  /// Backwards.
  Retrograde,
  /// Six of one, half a dozen the other.
  Undefined,
}
