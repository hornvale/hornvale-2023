use crate::action::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::system::action_processor::Data;
use crate::effect::*;
use anyhow::Error as AnyError;

/// The `LookAround` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAround {
  pub entity_id: EntityId,
}

impl Actionable for LookAround {
  fn get_effects(&self, _data: &mut Data) -> Result<Vec<Effect>, AnyError> {
    Ok(vec![
      create_effect!(EntityLooksAround {
        entity_id: self.entity_id,
      }),
      create_effect!(EntitySetInitiative {
        entity_id: self.entity_id,
        value: 0,
      }),
    ])
  }
}
