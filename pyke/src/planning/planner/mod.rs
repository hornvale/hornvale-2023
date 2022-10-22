use crate::planning::action::Action;
use crate::planning::error::Error;
use crate::planning::node::Node;
use crate::planning::nodes::Nodes;
use crate::planning::plan::Plan;
use crate::planning::state::State;

/// A planner, using the famous A* pathfinding algorithm.
#[derive(Clone, Debug)]
pub struct Planner {
  /// The start state.
  pub start: State,
  /// The goal state.
  pub goal: State,
  /// The open set.
  pub open: Nodes,
  /// The closed set.
  pub closed: Nodes,
  /// The action set.
  pub actions: Vec<Action>,
}

impl Planner {
  /// Constructor.
  #[named]
  pub fn new(start: State, goal: State, actions: Vec<Action>) -> Self {
    trace_enter!();
    trace_var!(start);
    trace_var!(goal);
    trace_var!(actions);
    let open = Nodes::new();
    let closed = Nodes::new();
    let result = Self {
      start,
      goal,
      open,
      closed,
      actions,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Reconstruct the plan.
  #[named]
  pub fn reconstruct_plan(&mut self, current: Node) -> Plan {
    trace_enter!();
    trace_var!(current);
    let start = self.start;
    trace_var!(start);
    let goal = self.goal;
    trace_var!(goal);
    let mut plan = Vec::new();
    let mut states = Vec::new();
    let mut pointer = current;
    loop {
      if let Some(action_name) = pointer.action_name {
        plan.push(action_name);
      }
      states.push(pointer.state);
      match pointer.parent_state {
        Some(parent_state) => {
          let index = self.closed.find_node_matching_state(&parent_state).unwrap();
          pointer = self.closed.nodes.swap_remove(index);
        },
        None => break,
      }
    }
    plan.reverse();
    trace_var!(plan);
    states.reverse();
    trace_var!(states);
    let length = plan.len();
    trace_var!(length);
    let result = Plan {
      start,
      goal,
      plan,
      states,
      length,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Plan!
  #[named]
  pub fn plan(&mut self) -> Result<Plan, Error> {
    trace_enter!();
    let node0 = Node::new_start(self.start, self.goal);
    trace_var!(node0);
    self.open.nodes.push(node0);
    loop {
      let current = self.open.take_cheapest_node()?;
      trace_var!(current);
      let at_goal = current.state.get_distance(&self.goal) == 0;
      trace_var!(at_goal);
      if at_goal {
        return Ok(self.reconstruct_plan(current));
      }
      self.closed.nodes.push(current.clone());
      let actions = self.get_possible_actions(&current.state);
      trace_var!(actions);
      for action in actions.iter() {
        let cost = current.g + action.cost;
        trace_var!(cost);
        let post_state = self.apply_action(action, &current.state);
        trace_var!(post_state);
        let mut open_index_result = self.open.find_node_matching_state(&post_state);
        trace_var!(open_index_result);
        let mut closed_index_result = self.closed.find_node_matching_state(&post_state);
        trace_var!(closed_index_result);
        if let Ok(open_index) = open_index_result {
          trace_var!(self.open.nodes[open_index]);
          if self.open.nodes[open_index].g > cost {
            let neighbor = self.open.nodes.swap_remove(open_index);
            trace!("Removed neighbor {:?} from open nodes.", neighbor);
            open_index_result = Err(Error::NotFound);
          }
        }
        if let Ok(closed_index) = closed_index_result {
          trace_var!(self.closed.nodes[closed_index]);
          if self.closed.nodes[closed_index].g > cost {
            let neighbor = self.closed.nodes.swap_remove(closed_index);
            trace!("Removed neighbor {:?} from closed nodes.", neighbor);
            closed_index_result = Err(Error::NotFound);
          }
        }
        if open_index_result.is_err() && closed_index_result.is_err() {
          let state = post_state;
          trace_var!(state);
          let parent_state = Some(current.state);
          trace_var!(parent_state);
          let g = cost;
          trace_var!(g);
          let h = post_state.get_distance(&self.goal);
          trace_var!(h);
          let f = g + h;
          trace_var!(f);
          let action_name = Some(action.name.clone());
          trace_var!(action_name);
          let neighbor = Node {
            state,
            parent_state,
            g,
            h,
            f,
            action_name,
          };
          trace_var!(neighbor);
          self.open.nodes.push(neighbor);
        }
      }
    }
  }

  /// Apply an action to the specified state and return the altered state.
  #[named]
  pub fn apply_action(&self, action: &Action, state: &State) -> State {
    trace_enter!();
    trace_var!(action);
    trace_var!(state);
    let postconditions = action.postconditions;
    let mask = postconditions.mask;
    let affected = mask ^ u64::MAX;
    let mut result = *state;
    result.values = (result.values & mask) | (postconditions.values & affected);
    result.mask &= postconditions.mask;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get possible state transitions.
  #[named]
  pub fn get_possible_actions(&self, from: &State) -> Vec<Action> {
    trace_enter!();
    trace_var!(from);
    let mut result = Vec::new();
    for action in self.actions.iter() {
      if action.preconditions.get_distance(from) == 0 {
        result.push(action.clone());
      }
    }
    trace_var!(result);
    trace_exit!();
    result
  }
}
