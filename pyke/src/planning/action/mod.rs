/// A model for describing an action.
///
/// The actual actions will be defined elsewhere; that's not our concern here.
///
/// All that concerns us is that an action has some effect on a modeled state.
pub trait Action {
  type CostType: Default;
  type StateType;
  type CostContextType;

  /// Calculate a cost for this action.
  fn get_cost(&self, state: &Self::StateType, context: &Self::CostContextType) -> Self::CostType;

  /// Check the preconditions for this action.
  fn check_preconditions(&self, state: &Self::StateType) -> bool;

  /// Apply the postconditions to this action.
  fn apply_postconditions(&self, state: &mut Self::StateType);
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
  struct AdderState(pub i32);

  #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
  struct AdderAction;

  impl Action for AdderAction {
    type CostType = u32;
    type StateType = AdderState;
    type CostContextType = ();
    fn get_cost(&self, _state: &AdderState, _context: &Self::CostContextType) -> Self::CostType {
      return 1;
    }
    fn check_preconditions(&self, state: &Self::StateType) -> bool {
      state.0 == 5
    }
    fn apply_postconditions(&self, state: &mut Self::StateType) {
      state.0 += 1;
    }
  }

  #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
  struct Planner;

  impl Planner {
    pub fn matches_preconditions(&self, state: &AdderState, action: &AdderAction) -> bool {
      action.check_preconditions(state)
    }
    pub fn apply(&self, state: &mut AdderState, action: &AdderAction) {
      action.apply_postconditions(state)
    }
  }

  #[named]
  #[test]
  pub fn test_default() {
    init();
    trace_enter!();
    let action = AdderAction;
    let mut state = AdderState(0);
    let planner = Planner;
    assert!(!planner.matches_preconditions(&state, &action));
    state.0 = 5;
    assert!(planner.matches_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.0, 6);
    state.0 = 5;
    assert!(planner.matches_preconditions(&state, &action));
    planner.apply(&mut state, &action);
    assert_eq!(state.0, 6);
    let cost = action.get_cost(&state, &());
    assert_eq!(cost, 1);
    trace_exit!();
  }
}
