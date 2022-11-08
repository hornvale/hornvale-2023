use anyhow::Error as AnyError;

use crate::action::Action;
use crate::ecs::entity::ObjectId;
use crate::ecs::entity::PlayerId;
use crate::map::Direction;

/// The `Command` enum.
///
/// This should provide (in a safe, serializable form), the specific command
/// and its parameters.
///
/// A command translates into either an in-character (IC) `Action` or an out-
/// of-character (OOC)... IDK what to call it.  These might be disparate enough
/// that there's not much point in trying to create a term that describes all
/// of them.
///
/// These objects are passed in `CommandEvent`s, through `EventChannel`s, from
/// the `ProcessInputSystem` to the `ProcessCommandSystem`, where they will be
/// converted into an `Action` or the appropriate code executed directly.
///
/// Commands have zero duration; they should be requested and transformed into
/// an action (if appropriate) in the same tick.  But it might be worth taking
/// control away from the player until a noninterruptible action completes, or
/// something like that.  E.g. moving from room to room may take a long time
/// in the wilderness, and we can simulate that by delaying the return of the
/// input prompt.
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
  GoDirection {
    player_id: PlayerId,
    direction: Direction,
    original_input: String,
  },
  LookAround {
    player_id: PlayerId,
    original_input: String,
  },
  LookAtObject {
    player_id: PlayerId,
    object_id: ObjectId,
    original_input: String,
  },
  LookDirection {
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
  pub fn get_action(&self) -> Result<Action, AnyError> {
    use Command::*;
    match self {
      LookAround { player_id, .. } => Ok(Action::LookAround {
        entity_id: (*player_id).into(),
      }),
      LookDirection {
        player_id,
        ref direction,
        ..
      } => Ok(Action::LookDirection {
        entity_id: (*player_id).into(),
        direction: *direction,
      }),
      LookAtObject {
        player_id,
        ref object_id,
        ..
      } => Ok(Action::LookAtObject {
        entity_id: (*player_id).into(),
        object_id: *object_id,
      }),
      GoDirection {
        player_id,
        ref direction,
        ..
      } => Ok(Action::GoDirection {
        entity_id: (*player_id).into(),
        direction: *direction,
      }),
      _ => Err(anyhow!("Unexpected!")),
    }
  }
}
