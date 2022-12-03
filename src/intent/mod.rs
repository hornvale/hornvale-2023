use crate::action::Action;
use crate::priority::Priority;

/// The `Intent` type.
///
/// This indicates the actor is currently determined to perform a specific
/// action.  It includes the action, its priority, and its initiative cost.
#[derive(Clone, Debug)]
pub struct Intent {
  /// The action to perform.
  pub action: Action,
  /// The priority of the action.
  pub priority: Priority,
  /// The initiative cost of the action.
  pub initiative_cost: usize,
}
