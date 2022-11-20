use crate::action::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::system::action_processor::Data;
use crate::effect::*;
use anyhow::Error;

/// The `LookAtEntity` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAtEntity {
  pub entity_id: EntityId,
  pub target_entity_id: EntityId,
}

impl Actionable for LookAtEntity {
  fn get_effects(&self, _data: &mut Data) -> Result<Vec<Effect>, Error> {
    Ok(vec![
      Effect::EntityLooksAtEntity(EntityLooksAtEntity {
        entity_id: self.entity_id,
        target_entity_id: self.target_entity_id,
      }),
      Effect::EntitySetInitiative(EntitySetInitiative {
        entity_id: self.entity_id,
        value: 0,
      }),
    ])
  }
}
