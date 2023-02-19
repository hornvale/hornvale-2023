use crate::action::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::AllData;
use crate::effect::*;
use anyhow::Error;

/// The `LookAtEntity` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAtEntity {
  pub entity_id: EntityId,
  pub target_entity_id: EntityId,
}

impl Actionable for LookAtEntity {
  fn get_actor_entity_id(&self) -> EntityId {
    self.entity_id
  }

  fn get_effects(&self, _data: &mut AllData) -> Result<Vec<Effect>, Error> {
    Ok(vec![
      create_effect!(EntityLooksAtEntity {
        entity_id: self.entity_id,
        target_entity_id: self.target_entity_id,
      }),
      create_effect!(EntitySetInitiative {
        entity_id: self.entity_id,
        value: 0,
      }),
    ])
  }
}
