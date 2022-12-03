use crate::action::Action;
use crate::goap::state::State;

/// An action option.
#[derive(Clone, Debug)]
pub struct ActionOption {
  /// The represented action.
  pub action: Action,
  /// The cost of this action.
  pub cost: usize,
  /// The expectations of this action of the state.
  pub preconditions: State,
  /// The expected modifications of this action to the state.
  pub postconditions: State,
}
