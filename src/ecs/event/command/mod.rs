use crate::command::Command as CommandObject;

/// The `CommandEvent` type.
///
/// This represents a command executed by a player.
#[derive(Clone, Debug)]
pub struct Command {
  pub command: CommandObject,
}
