use crate::action::Action;
use crate::command::Commandable;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Quit` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Quit {
  pub player_id: PlayerId,
  pub original_input: String,
}

impl Commandable for Quit {
  fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    panic!("WTF");
    // Ok(None)
  }
}
