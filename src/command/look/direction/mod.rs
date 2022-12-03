use crate::action::*;
use crate::command::Commandable;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use crate::map::Direction;
use anyhow::Error;

/// The `LookDirection` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookDirection {
  pub player_id: PlayerId,
  pub direction: Direction,
  pub original_input: String,
}

impl Commandable for LookDirection {
  fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(create_action!(LookDirectionAction {
      entity_id: self.player_id.into(),
      direction: self.direction,
    })))
  }
}
