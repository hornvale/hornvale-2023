use crate::action::Action;
use crate::action::LookAroundAction;
use crate::command::Commandable;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `LookAround` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAround {
  pub player_id: PlayerId,
  pub original_input: String,
}

impl Commandable for LookAround {
  fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(create_action!(LookAroundAction {
      entity_id: self.player_id.into(),
    })))
  }
}
