use crate::action::Action;
use crate::action::LookAroundAction;
use crate::entity::PlayerId;
use crate::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `LookAround` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAround {
  pub player_id: PlayerId,
  pub original_input: String,
}

impl LookAround {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(Action::LookAround(LookAroundAction {
      entity_id: self.player_id.into(),
    })))
  }
}
