use specs::prelude::*;

pub mod constants;
pub use constants::*;

/// The `Initiative` object.
///
/// Initiative is used to limit the rate at which an actor can act.
///
/// Initiative is doled out on a per-tick basis.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Initiative {
  /// The current initiative level.
  pub current: usize,
  /// The refill rate for initiative.
  pub increment: usize,
}
