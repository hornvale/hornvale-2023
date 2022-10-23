use crate::planning::state::State;

/// An individual A* node.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Node {
  /// The state represented by this node.
  pub state: State,
  /// The parent state.
  pub parent_state: Option<State>,
  /// g+h.
  pub f: usize,
  /// The cost so far.
  pub g: usize,
  /// Heuristic for remaining cost.
  pub h: usize,
  /// The name of the action.
  pub action_name: Option<String>,
}

impl Node {
  /// Constructor for start node.
  #[named]
  pub fn new_start(state: State, goal: State) -> Self {
    trace_enter!();
    trace_var!(state);
    let parent_state = None;
    trace_var!(parent_state);
    let action_name = None;
    trace_var!(action_name);
    let g = 0;
    trace_var!(g);
    let result = Self::new(state, parent_state, goal, action_name, g);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Constructor.
  #[named]
  pub fn new(state: State, parent_state: Option<State>, goal: State, action_name: Option<String>, g: usize) -> Self {
    trace_enter!();
    trace_var!(state);
    trace_var!(parent_state);
    trace_var!(goal);
    trace_var!(action_name);
    trace_var!(g);
    let h = state.get_distance(&goal);
    trace_var!(h);
    let f = g + h;
    trace_var!(f);
    let result = Self {
      state,
      parent_state,
      f,
      g,
      h,
      action_name,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_new_start() {
    init();
    trace_enter!();
    let start = State::default();
    let goal = State::default();
    let node = Node::new_start(start, goal);
    print_var!(node);
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test_new() {
    init();
    trace_enter!();
    let start = State::default();
    let goal = State::default();
    let node = Node::new(start, None, goal, None, 0);
    print_var!(node);
    trace_exit!();
  }
}
