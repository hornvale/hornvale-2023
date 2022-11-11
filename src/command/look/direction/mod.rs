use crate::action::*;
use crate::ecs::entity::PlayerId;
use crate::ecs::systems::command_processor::Data as CommandProcessorData;
use crate::map::Direction;
use anyhow::Error;

/// The `LookDirection` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookDirection {
  pub player_id: PlayerId,
  pub direction: Direction,
  pub original_input: String,
}

impl LookDirection {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(Action::LookDirection(LookDirectionAction {
      entity_id: self.player_id.into(),
      direction: self.direction,
    })))
  }
}
