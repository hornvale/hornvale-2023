use crate::planning::error::Error;
use crate::planning::node::Node;
use crate::planning::state::State;

/// The `Nodes` type.
///
/// This wraps a list of `Node` objects.
#[derive(Clone, Debug)]
pub struct Nodes {
  /// The list of nodes.
  pub nodes: Vec<Node>,
}

impl Nodes {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let nodes = Vec::new();
    trace_var!(nodes);
    let result = Self { nodes };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Index of node containing matching world state, if any.
  #[named]
  pub fn find_node_matching_state(&self, state: &State) -> Result<usize, Error> {
    trace_enter!();
    trace_var!(state);
    let result = {
      for (index, node) in self.nodes.iter().enumerate() {
        if node.state == *state {
          return Ok(index);
        }
      }
      Err(Error::NotFound)
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Index of node with the lowest f value.
  #[named]
  pub fn find_cheapest_node(&self) -> Result<usize, Error> {
    trace_enter!();
    let mut lowest_value = usize::MAX;
    trace_var!(lowest_value);
    let mut result = Err(Error::NotFound);
    for (index, node) in self.nodes.iter().enumerate() {
      if node.f < lowest_value {
        result = Ok(index);
        lowest_value = node.f;
      }
    }
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Take cheapest node.
  #[named]
  pub fn take_cheapest_node(&mut self) -> Result<Node, Error> {
    trace_enter!();
    let index = self.find_cheapest_node()?;
    trace_var!(index);
    let result = Ok(self.nodes.swap_remove(index));
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Default for Nodes {
  fn default() -> Self {
    Self::new()
  }
}
