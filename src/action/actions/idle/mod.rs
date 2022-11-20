use super::super::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::system::action_processor::Data;
use crate::effect::*;
use anyhow::Error as AnyError;

/// The `Idle` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Idle {
  pub entity_id: EntityId,
}

impl Actionable for Idle {
  fn get_effects(&self, _data: &mut Data) -> Result<Vec<Effect>, AnyError> {
    Ok(vec![Effect::EntitySetInitiative(EntitySetInitiative {
      entity_id: self.entity_id,
      value: 0,
    })])
  }
}
