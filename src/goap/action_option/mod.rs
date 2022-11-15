use crate::goap::state::State;

/// An action option.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionOption {
  /// The name of this action.
  pub name: String,
  /// The cost of this action.
  pub cost: usize,
  /// The expectations of this action of the state.
  pub preconditions: State,
  /// The expected modifications of this action to the state.
  pub postconditions: State,
}
