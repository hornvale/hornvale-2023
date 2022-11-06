use crate::command::Command as CommandObject;

/// The `CommandEvent` type.
///
/// This represents a command executed by a player.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
  pub command: CommandObject,
}
