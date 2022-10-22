use pyke::planning::action::Action;
use pyke::planning::planner::Planner;
use pyke::planning::state::State;

use pretty_env_logger::env_logger::builder;
use std::env::set_var;
use volmark::*;

fn init() {
  let _ = builder().is_test(true).try_init();
  set_var("RUST_BACKTRACE", "1");
}

#[test]
#[named]
fn test_1_action_plan() {
  init();
  trace_enter!();
  let setbit0_action = Action {
    name: "Set Bit 0".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
    },
  };
  trace_var!(setbit0_action);
  let start = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
  };
  trace_var!(start);
  let goal = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
  };
  trace_var!(goal);
  let actions = vec![setbit0_action.clone()];
  trace_var!(actions);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), 1);
  assert_eq!(plan.plan, vec![setbit0_action.name]);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}

#[test]
#[named]
fn test_2_action_plan() {
  init();
  trace_enter!();
  let setbit0_action = Action {
    name: "Set Bit 0".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
    },
  };
  trace_var!(setbit0_action);
  let setbit1_action = Action {
    name: "Set Bit 1".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1101,
    },
  };
  trace_var!(setbit1_action);
  let start = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1100,
  };
  trace_var!(start);
  let goal = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1100,
  };
  trace_var!(goal);
  let actions = vec![setbit0_action.clone(), setbit1_action.clone()];
  trace_var!(actions);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), 2);
  assert_eq!(plan.plan, vec![setbit0_action.name, setbit1_action.name,]);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}

#[test]
#[named]
fn test_3_action_plan() {
  init();
  trace_enter!();
  let setbit0_action = Action {
    name: "Set Bit 0".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
    },
  };
  trace_var!(setbit0_action);
  let setbit1_action = Action {
    name: "Set Bit 1".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1101,
    },
  };
  trace_var!(setbit1_action);
  let setbit2_action = Action {
    name: "Set Bit 2".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1011,
    },
  };
  trace_var!(setbit2_action);
  let start = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1000,
  };
  trace_var!(start);
  let goal = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0111,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1000,
  };
  trace_var!(goal);
  let actions = vec![setbit0_action.clone(), setbit1_action.clone(), setbit2_action.clone()];
  trace_var!(actions);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), 3);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}

#[test]
#[named]
fn test_4_action_plan() {
  init();
  trace_enter!();
  let setbit0_action = Action {
    name: "Set Bit 0".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110,
    },
  };
  trace_var!(setbit0_action);
  let setbit1_action = Action {
    name: "Set Bit 1".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1101,
    },
  };
  trace_var!(setbit1_action);
  let setbit2_action = Action {
    name: "Set Bit 2".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1011,
    },
  };
  trace_var!(setbit2_action);
  let setbit3_action = Action {
    name: "Set Bit 3".to_string(),
    cost: 1,
    preconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    },
    postconditions: State {
      values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000,
      mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0111,
    },
  };
  trace_var!(setbit3_action);
  let start = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
  };
  trace_var!(start);
  let goal = State {
    values: 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111,
    mask: 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
  };
  trace_var!(goal);
  let actions = vec![
    setbit0_action.clone(),
    setbit1_action.clone(),
    setbit2_action.clone(),
    setbit3_action.clone(),
  ];
  trace_var!(actions);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), 4);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}

#[test]
#[named]
fn test_many_simple_action_plan() {
  init();
  trace_enter!();
  let limit = 63;
  let mut actions = Vec::new();
  for i in 0..limit {
    let action = Action {
      name: format!("Set Bit {}", i),
      cost: 1,
      preconditions: State {
        values: 0,
        mask: u64::MAX,
      },
      postconditions: State {
        values: 1 << i,
        mask: !(1 << i),
      },
    };
    actions.push(action);
    actions.reverse();
  }
  trace_var!(actions);
  let start = State { values: 0, mask: 0 };
  trace_var!(start);
  let goal = State {
    values: (1 << limit) - 1,
    mask: !((1 << limit) - 1),
  };
  trace_var!(goal);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  println!("{:#?}", plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), limit as usize);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}

#[test]
#[named]
fn test_many_complex_action_plan() {
  init();
  trace_enter!();
  let limit = 63;
  let mut actions = Vec::new();
  for i in 0..limit {
    let mut precondition_values: u64 = 0;
    let mut counter = 1 << i;
    while counter > 1 {
      counter = counter >> 1;
      precondition_values = precondition_values | counter;
    }
    let action = Action {
      name: format!("Set Bit {}", i),
      cost: precondition_values.count_ones() as usize,
      preconditions: State {
        values: precondition_values,
        mask: !precondition_values,
      },
      postconditions: State {
        values: 1 << i,
        mask: !(1 << i),
      },
    };
    actions.push(action);
    actions.reverse();
  }
  trace_var!(actions);
  let start = State { values: 0, mask: 0 };
  trace_var!(start);
  let goal = State {
    values: (1 << limit) - 1,
    mask: !((1 << limit) - 1),
  };
  trace_var!(goal);
  let mut planner = Planner::new(start, goal, actions);
  trace_var!(planner);
  let plan = planner.plan().unwrap();
  print_var!(plan);
  println!("{:#?}", plan);
  assert_eq!(plan.start, start);
  assert_eq!(plan.goal, goal);
  assert_eq!(plan.plan.len(), limit as usize);
  assert_eq!(plan.states.len(), plan.plan.len() + 1);
}
