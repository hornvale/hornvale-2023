/// A simple test sense event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SenseEvent {
  /// Whether a red light is on or not.
  RedLightStatus(bool),
}
