use crate::action::{Action, IdleAction};
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Idle` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Idle {
  pub player_id: PlayerId,
  pub original_input: String,
}

impl Idle {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(Action::Idle(IdleAction {
      entity_id: self.player_id.into(),
    })))
  }
}
