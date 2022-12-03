use crate::action::Action;
use crate::ecs::system::command_processor::Data;
use anyhow::Error;
use std::fmt::Debug;

/// The `Commandable` trait.
pub trait Commandable: Debug + Send + Sync {
  /// Execute the extra-diegetic command or return the intra-diegetic action.
  fn get_action(&self, data: &mut Data) -> Result<Option<Action>, Error>;
}
