use super::super::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::AllData;
use crate::effect::*;
use anyhow::Error as AnyError;

/// The `Idle` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Idle {
  pub entity_id: EntityId,
}

impl Actionable for Idle {
  fn get_actor_entity_id(&self) -> EntityId {
    self.entity_id
  }

  fn get_effects(&self, _data: &mut AllData) -> Result<Vec<Effect>, AnyError> {
    Ok(vec![create_effect!(EntitySetInitiative {
      entity_id: self.entity_id,
      value: 0,
    })])
  }
}
