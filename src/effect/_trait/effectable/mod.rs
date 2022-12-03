use crate::ecs::system::effect_processor::Data;
use anyhow::Error;
use std::fmt::Debug;

/// The `Effectable` trait.
///
/// This is what defines an Effect.
pub trait Effectable: Debug + Send + Sync {
  /// Process the effect's changes on the world.
  fn process(&self, data: &mut Data) -> Result<(), Error>;
}
