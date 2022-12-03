use crate::action::Action;
use crate::goap::state::State;

/// The `Plan` type.
///
/// This includes a list of actions to undertake.
#[derive(Clone, Debug)]
pub struct Plan {
  /// The start state.
  pub start: State,
  /// The goal state.
  pub goal: State,
  /// The plan itself.
  pub plan: Vec<Action>,
  /// All intermediate world states.
  pub states: Vec<State>,
  /// Number of steps.
  pub length: usize,
}
