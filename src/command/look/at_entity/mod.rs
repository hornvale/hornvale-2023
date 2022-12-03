use crate::action::Action;
use crate::action::LookAtEntityAction;
use crate::ecs::entity::EntityId;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `LookAtEntity` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAtEntity {
  pub player_id: PlayerId,
  pub target_entity_id: EntityId,
  pub original_input: String,
}

impl LookAtEntity {
  pub fn get_action(&self, _data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    Ok(Some(create_action!(LookAtEntityAction {
      entity_id: self.player_id.into(),
      target_entity_id: self.target_entity_id,
    })))
  }
}
