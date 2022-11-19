use crate::action::{Action, GoDirectionAction, LookAroundAction, LookDirectionAction};
use crate::ecs::entity::EntityId;
use crate::ecs::system::ai_processor::Data;
use crate::goap::ActionOption;
use crate::goap::Planner;
use crate::goap::State;
use crate::map::Direction;
use anyhow::Error as AnyError;
use rand::prelude::*;

/// The `MoveRandomly` type.
///
/// Each variant is a distinct engine for controlling movement.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MoveRandomly {}

impl MoveRandomly {
  /// Retrieve an action for this AI.
  pub fn get_action(&self, entity_id: EntityId, data: &mut Data) -> Result<Option<Action>, AnyError> {
    let entity = get_entity!(data, entity_id);
    let has_initiative = get_has_initiative!(data, entity).unwrap();
    if has_initiative.current > 250 {
      let direction: Direction = data.random_resource.0.gen();
      let mask = 3;
      let action_options = vec![
        ActionOption {
          action: Action::LookDirection(LookDirectionAction { entity_id, direction }),
          cost: 1,
          preconditions: State { values: 0b00, mask },
          postconditions: State { values: 0b01, mask },
        },
        ActionOption {
          action: Action::GoDirection(GoDirectionAction { entity_id, direction }),
          cost: 1,
          preconditions: State { values: 0b01, mask },
          postconditions: State { values: 0b10, mask },
        },
        ActionOption {
          action: Action::LookAround(LookAroundAction { entity_id }),
          cost: 1,
          preconditions: State { values: 0b10, mask },
          postconditions: State { values: 0b00, mask },
        },
      ];
      let mut start_state = *get_state!(data, entity).unwrap();
      start_state.mask = mask;
      let values = match start_state.values {
        0b10 => 0b00,
        _ => 0b10,
      };
      let dest_state = State { values, mask };
      let mut planner = Planner::new(start_state, dest_state, action_options);
      let plan = planner.plan()?;
      // println!("{:#?}\n\r", plan);
      let action = plan.plan.first().unwrap().clone();
      // println!("{:#?}\n\r", action);
      return Ok(Some(action));
    }
    Ok(None)
  }
}
