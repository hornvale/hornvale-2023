use crate::action::*;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use crate::map::Direction;
use anyhow::Error;

/// The `GoDirection` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GoDirection {
  pub player_id: PlayerId,
  pub direction: Direction,
  pub original_input: String,
}

impl GoDirection {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(Action::GoDirection(GoDirectionAction {
      entity_id: self.player_id.into(),
      direction: self.direction,
    })))
  }
}
