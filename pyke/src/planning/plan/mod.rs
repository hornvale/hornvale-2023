use crate::planning::state::State;

/// The `Plan` type.
///
/// This includes a list of actions to undertake.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
  /// The start state.
  pub start: State,
  /// The goal state.
  pub goal: State,
  /// The plan itself.
  pub plan: Vec<String>,
  /// All intermediate world states.
  pub states: Vec<State>,
  /// Number of steps.
  pub length: usize,
}
