use crate::entity::EntityId;
use crate::system::action_processor::Data as ActionProcessorData;
use anyhow::Error;

/// The `Idle` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Idle {
  pub entity_id: EntityId,
}

impl Idle {
  pub fn execute(&self, _data: &mut ActionProcessorData) -> Result<(), Error> {
    Ok(())
  }
}
