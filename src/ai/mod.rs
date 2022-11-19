use crate::action::Action;
use crate::entity::EntityId;
use crate::system::ai_processor::Data;
use anyhow::Error as AnyError;

pub mod move_randomly;
pub use move_randomly::MoveRandomly;

/// The `Ai` enum.
///
/// Each variant is a distinct engine for controlling movement.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Ai {
  MoveRandomly(MoveRandomly),
}

impl Ai {
  /// Retrieve an action for this AI.
  pub fn get_action(&self, entity_id: EntityId, data: &mut Data) -> Result<Option<Action>, AnyError> {
    use Ai::*;
    match self {
      MoveRandomly(engine) => Ok(engine.get_action(entity_id, data)?),
    }
  }
}
