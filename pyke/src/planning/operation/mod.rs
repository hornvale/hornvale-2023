use crate::planning::action::Action;
use crate::planning::state::State;
use std::default::Default;
use std::fmt::Debug;
use std::ops::AddAssign;

/// The `Operation` enum.
///
/// An operation is either an `Action` or a `Sequence`. An `Action` wraps a
/// single `Action` trait object, while a `Sequence` wraps a sequence of
/// `Operation` objects.
pub enum Operation<C, S, X> {
  Action(Box<dyn Action<CostType = C, StateType = S, CostContextType = X>>),
  Sequence(Vec<Operation<C, S, X>>),
}

impl<C: Debug + Default + AddAssign, S: Clone + Debug + Default + State, X> Action for Operation<C, S, X> {
  type CostType = C;
  type StateType = S;
  type CostContextType = X;

  #[named]
  fn get_cost(&self, state: &Self::StateType, context: &Self::CostContextType) -> Self::CostType {
    trace_enter!();
    use Operation::*;
    let result = match &self {
      Action(boxed_action) => boxed_action.get_cost(state, context),
      Sequence(operation_vector) => {
        let mut result = Self::CostType::default();
        for operation in operation_vector.iter() {
          result += operation.get_cost(state, context);
        }
        result
      },
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn check_preconditions(&self, state: &Self::StateType) -> bool {
    trace_enter!();
    use Operation::*;
    let result = match &self {
      Action(boxed_action) => boxed_action.check_preconditions(state),
      Sequence(operation_vector) => {
        let mut result = true;
        let mut test_state = state.clone();
        for operation in operation_vector.iter().rev() {
          result = result && operation.check_preconditions(&test_state);
          if !result {
            break;
          }
          operation.apply_postconditions(&mut test_state);
        }
        result
      },
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn apply_postconditions(&self, state: &mut Self::StateType) {
    trace_enter!();
    use Operation::*;
    match &self {
      Action(boxed_action) => boxed_action.apply_postconditions(state),
      Sequence(operation_vector) => {
        for operation in operation_vector.iter() {
          operation.apply_postconditions(state);
        }
      },
    }
    trace_exit!();
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
  struct SimpleState {
    pub a: Option<u32>,
    pub b: Option<u32>,
    pub c: Option<u32>,
  }

  impl SimpleState {
    fn get_composite(&self, state: &SimpleState) -> Self {
      let a = state.a.or(self.a);
      let b = state.b.or(self.b);
      let c = state.c.or(self.c);
      SimpleState { a, b, c }
    }
  }

  impl State for SimpleState {}

  #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
  struct AddAAction;

  impl Action for AddAAction {
    type CostType = u32;
    type StateType = SimpleState;
    type CostContextType = ();
    fn get_cost(&self, _state: &Self::StateType, _context: &Self::CostContextType) -> Self::CostType {
      return 1;
    }
    fn check_preconditions(&self, state: &Self::StateType) -> bool {
      state.a.is_some()
    }
    fn apply_postconditions(&self, state: &mut Self::StateType) {
      if let Some(ref mut number) = state.a {
        *number = *number + 1;
      }
    }
  }

  #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
  struct Planner;

  impl Planner {
    pub fn check_preconditions(&self, state: &SimpleState, action: &impl Action<StateType = SimpleState>) -> bool {
      action.check_preconditions(state)
    }
    pub fn apply(&self, state: &mut SimpleState, action: &impl Action<StateType = SimpleState>) {
      action.apply_postconditions(state)
    }
  }

  #[named]
  #[test]
  pub fn test1() {
    init();
    trace_enter!();
    let action = AddAAction;
    let a = None;
    let b = None;
    let c = None;
    let mut state = SimpleState { a, b, c };
    let planner = Planner;
    assert!(!planner.check_preconditions(&state, &action));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(6));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(6));
    let cost = action.get_cost(&state, &());
    assert_eq!(cost, 1);
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test2() {
    init();
    trace_enter!();
    let action = Operation::Action(Box::new(AddAAction));
    let a = None;
    let b = None;
    let c = None;
    let mut state = SimpleState { a, b, c };
    let planner = Planner;
    assert!(!planner.check_preconditions(&state, &action));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(6));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(6));
    let cost = action.get_cost(&state, &());
    assert_eq!(cost, 1);
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test3() {
    init();
    trace_enter!();
    let action = Operation::Sequence(vec![
      Operation::Action(Box::new(AddAAction)),
      Operation::Action(Box::new(AddAAction)),
      Operation::Action(Box::new(AddAAction)),
      Operation::Action(Box::new(AddAAction)),
      Operation::Action(Box::new(AddAAction)),
    ]);
    let a = None;
    let b = None;
    let c = None;
    let mut state = SimpleState { a, b, c };
    let planner = Planner;
    assert!(!planner.check_preconditions(&state, &action));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(10));
    state.a = Some(5);
    assert!(planner.check_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.a, Some(10));
    let cost = action.get_cost(&state, &());
    assert_eq!(cost, 5);
    trace_exit!();
  }
}
