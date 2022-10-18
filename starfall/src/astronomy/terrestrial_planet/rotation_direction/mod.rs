/// Direction of rotation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RotationDirection {
  /// Forwards.
  Prograde,
  /// Backwards.
  Retrograde,
  /// Six of one, half a dozen the other.
  Undefined,
}
