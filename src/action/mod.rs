use crate::ecs::entity::EntityId;
use crate::ecs::AllData;
use crate::effect::Effect;
use anyhow::Error as AnyError;
use std::sync::Arc;

pub mod _trait;
pub use _trait::actionable::Actionable;
pub mod actions;
pub use actions::*;

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
#[derive(Clone, Debug)]
pub struct Action(pub Arc<dyn Actionable>);

impl Actionable for Action {
  /// Get the actor entity ID.
  fn get_actor_entity_id(&self) -> EntityId {
    (*self.0).get_actor_entity_id()
  }

  /// Get the predicted effects of this action.
  fn get_effects(&self, data: &mut AllData) -> Result<Vec<Effect>, AnyError> {
    (*self.0).get_effects(data)
  }

  /// Can this action be executed?
  fn can_execute(&self, data: &mut AllData) -> Result<(), AnyError> {
    (*self.0).can_execute(data)
  }

  /// Execute the action.
  fn execute(&self, data: &mut AllData) -> Result<(), AnyError> {
    (*self.0).can_execute(data)?;
    (*self.0).execute(data)
  }
}
