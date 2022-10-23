/// The RotationDirection
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RotationDirection {
  /// Forwards.
  Prograde,
  /// Backwardss.
  Retrograde,
  /// WTF
  Undefined,
}
