use crate::ecs::system::action_processor::Data;
use crate::effect::Effect;
use anyhow::Error as AnyError;
use std::fmt::Debug;

/// The `Actionable` trait.
pub trait Actionable: Debug + Send + Sync {
  /// Returns a list of effects to which this action can be expected to lead.
  ///
  /// These will be created on an individual basis based on the data available.
  fn get_effects(&self, data: &mut Data) -> Result<Vec<Effect>, AnyError>;

  /// Indicates whether this action can be executed.
  ///
  /// This is highly context-sensitive.  If Ok(()), no further information is
  /// needed or supplied.  If the action cannot be executed, an error will be
  /// returned.
  fn can_execute(&self, _data: &mut Data) -> Result<(), AnyError> {
    Ok(())
  }

  /// Execute; that is, perform final checks and transform into effects.
  fn execute(&self, data: &mut Data) -> Result<(), AnyError> {
    for effect in self.get_effects(data)? {
      write_effect_event!(data, effect);
    }
    Ok(())
  }
}
