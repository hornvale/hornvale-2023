use crate::action::Action;
use crate::command::Commandable;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Echo` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Echo {
  pub player_id: PlayerId,
  pub string: String,
  pub original_input: String,
}

impl Commandable for Echo {
  fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    write_output_event!(data, self.string.clone());
    Ok(None)
  }
}
