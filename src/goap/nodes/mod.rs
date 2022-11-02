use crate::goap::error::Error;
use crate::goap::node::Node;
use crate::goap::state::State;

/// The `Nodes` type.
///
/// This wraps a list of `Node` objects.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nodes {
  /// The list of nodes.
  pub nodes: Vec<Node>,
}

impl Nodes {
  /// Constructor.

  pub fn new() -> Self {
    let nodes = Vec::new();

    Self { nodes }
  }

  /// Index of node containing matching world state, if any.

  pub fn find_node_matching_state(&self, state: &State) -> Result<usize, Error> {
    {
      for (index, node) in self.nodes.iter().enumerate() {
        if node.state == *state {
          return Ok(index);
        }
      }
      Err(Error::NotFound)
    }
  }

  /// Index of node with the lowest f value.

  pub fn find_cheapest_node(&self) -> Result<usize, Error> {
    let mut lowest_value = usize::MAX;
    let mut result = Err(Error::NotFound);
    for (index, node) in self.nodes.iter().enumerate() {
      if node.f < lowest_value {
        result = Ok(index);
        lowest_value = node.f;
      }
    }

    result
  }

  /// Take cheapest node.

  pub fn take_cheapest_node(&mut self) -> Result<Node, Error> {
    let index = self.find_cheapest_node()?;

    Ok(self.nodes.swap_remove(index))
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test1() {
    init();
    let start = State::default();
    let mut nodes = Nodes::default();
    assert_eq!(nodes.find_node_matching_state(&start), Err(Error::NotFound));
    assert_eq!(nodes.find_cheapest_node(), Err(Error::NotFound));
    assert_eq!(nodes.take_cheapest_node(), Err(Error::NotFound));
    print_var!(nodes);
  }

  #[test]
  pub fn test2() {
    init();
    let start = State::default();
    let goal = State::default();
    let node = Node::new_start(start, goal);
    let mut nodes = Nodes::default();
    assert_eq!(nodes.find_node_matching_state(&start), Err(Error::NotFound));
    assert_eq!(nodes.find_cheapest_node(), Err(Error::NotFound));
    assert_eq!(nodes.take_cheapest_node(), Err(Error::NotFound));
    nodes.nodes.push(node.clone());
    assert_eq!(nodes.find_node_matching_state(&start), Ok(0));
    assert_eq!(nodes.find_cheapest_node(), Ok(0));
    assert_eq!(nodes.take_cheapest_node(), Ok(node));
    print_var!(nodes);
  }
}
