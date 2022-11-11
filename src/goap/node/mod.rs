use crate::goap::state::State;

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
  pub fn new_start(state: State, goal: State) -> Self {
    let parent_state = None;
    let action_name = None;
    let g = 0;

    Self::new(state, parent_state, goal, action_name, g)
  }

  /// Constructor.
  pub fn new(state: State, parent_state: Option<State>, goal: State, action_name: Option<String>, g: usize) -> Self {
    let h = state.get_distance(&goal);
    let f = g + h;

    Self {
      state,
      parent_state,
      f,
      g,
      h,
      action_name,
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_new_start() {
    init();
    let start = State::default();
    let goal = State::default();
    let node = Node::new_start(start, goal);
    print_var!(node);
  }

  #[test]
  pub fn test_new() {
    init();
    let start = State::default();
    let goal = State::default();
    let node = Node::new(start, None, goal, None, 0);
    print_var!(node);
  }
}
