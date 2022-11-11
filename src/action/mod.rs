use crate::ecs::systems::action_processor::Data as ActionProcessorData;
use anyhow::Error;

pub mod go_direction;
pub use go_direction::GoDirection as GoDirectionAction;
pub mod look;
pub use look::*;

/// The `Action` enum.
///
/// This enum just captures (in a safe, serializeable form) the specific action
/// and its parameters.
///
/// Actions are fallible and subjective, representing an _attempt_ to perform
/// the action, not the effect of the action.  For the latter, see `Effect`.
///
/// Events originate in several ways:
/// - User input is translated into `Action` objects via `ProcessCommandSystem`.
/// - NPC AI will typically emit an `Action` after making a decision.
/// - Various systems may emit an `Action` to automate processes while ensuring
///   fallibility.
///
/// `Action`s can have a nonzero duration, which is to say that they will take
/// longer than one tick to accomplish.  This has implications:
/// - actions can be cancelled by the actor.
/// - actions can be interrupted by changing situations, hostile actions, etc.
/// - actions may need to be reported to observers.
/// - actions' effects can be divided over their duration, discretely or con-
///   tinuously.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Action {
  /// Go in a specific direction.  This should respect current movement method
  /// (e.g. walking, flying, etc).
  GoDirection(GoDirectionAction),
  /// Look at the current room.  This should provide a snapshot the sensory
  /// data of the room.
  LookAround(LookAroundAction),
  /// Look at a specific object, either on the actor or in their environment.
  LookAtEntity(LookAtEntityAction),
  /// Look through the passage in the specified direction.  Will not work with
  /// closed doors.  Certain other passageways may prevent the action as well.
  LookDirection(LookDirectionAction),
}

impl Action {
  pub fn execute(&self, data: &mut ActionProcessorData) -> Result<(), Error> {
    use Action::*;
    match &self {
      GoDirection(action) => action.execute(data)?,
      LookAround(action) => action.execute(data)?,
      LookAtEntity(action) => action.execute(data)?,
      LookDirection(action) => action.execute(data)?,
    }
    Ok(())
  }
}
