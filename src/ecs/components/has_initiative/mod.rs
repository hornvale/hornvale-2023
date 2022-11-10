use specs::prelude::*;

/// The `HasInitiative` type.
#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HasInitiative {
  /// The current "initiative" energy that the object has.
  pub current: usize,
  /// The rate at which this object is given "initiative energy".
  pub refill_rate: usize,
}
