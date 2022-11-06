use anyhow::Error as AnyError;

use crate::action::Action;
use crate::ecs::entity::ObjectId;
use crate::ecs::entity::PlayerId;
use crate::map::Direction;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Command {
  Echo {
    player_id: PlayerId,
    string: String,
    original_input: String,
  },
  Eval {
    player_id: PlayerId,
    string: String,
    original_input: String,
  },
  Look {
    player_id: PlayerId,
    original_input: String,
  },
  LookDirection {
    player_id: PlayerId,
    direction: Direction,
    original_input: String,
  },
  LookAtObject {
    player_id: PlayerId,
    object_id: ObjectId,
    original_input: String,
  },
  MoveDirection {
    player_id: PlayerId,
    direction: Direction,
    original_input: String,
  },
  Quit {
    player_id: PlayerId,
    original_input: String,
  },
}

impl Command {
  /// Retrieve an action for this command.
  pub fn get_action(self) -> Result<Action, AnyError> {
    use Command::*;
    match self {
      Look { player_id, .. } => Ok(Action::Look {
        entity_id: player_id.into(),
      }),
      LookDirection {
        player_id,
        ref direction,
        ..
      } => Ok(Action::LookDirection {
        entity_id: player_id.into(),
        direction: *direction,
      }),
      LookAtObject {
        player_id,
        ref object_id,
        ..
      } => Ok(Action::LookAtObject {
        entity_id: player_id.into(),
        object_id: *object_id,
      }),
      MoveDirection {
        player_id,
        ref direction,
        ..
      } => Ok(Action::MoveDirection {
        entity_id: player_id.into(),
        direction: *direction,
      }),
      _ => Err(anyhow!("Unexpected!")),
    }
  }
}
