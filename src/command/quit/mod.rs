use crate::action::Action;
use crate::ecs::entity::PlayerId;
use crate::ecs::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Quit` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Quit {
  pub player_id: PlayerId,
  pub original_input: String,
}

impl Quit {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    panic!("WTF");
    // Ok(None)
  }
}