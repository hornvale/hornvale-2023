use crate::action::Action;
use crate::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error as AnyError;

pub mod echo;
pub use echo::Echo as EchoCommand;
pub mod eval;
pub use eval::Eval as EvalCommand;
pub mod go_direction;
pub use go_direction::GoDirection as GoDirectionCommand;
pub mod look;
pub use look::*;
pub mod quit;
pub use quit::Quit as QuitCommand;

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
  Echo(EchoCommand),
  Eval(EvalCommand),
  GoDirection(GoDirectionCommand),
  LookAround(LookAroundCommand),
  LookAtEntity(LookAtEntityCommand),
  LookDirection(LookDirectionCommand),
  Quit(QuitCommand),
}

impl Command {
  /// Retrieve an action for this command, or evaluate it.
  ///
  /// Commands come in two forms: extradiagetic and intradiagetic.
  ///
  /// Extradiagetic or out-of-character/OOC commands operate outside the game
  /// world.  They generally perform some operation on game state and return
  /// directly.
  ///
  /// Intradiagetic or in-character/IC commands are translated to an Action
  /// object that represents the in-game action to take.
  ///
  /// Thus extradiagetic commands will return None here, and intradiagetic
  /// commands will return Some(action).
  pub fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, AnyError> {
    use Command::*;
    match self {
      Echo(command) => Ok(command.get_action(data)?),
      Eval(command) => Ok(command.get_action(data)?),
      GoDirection(command) => Ok(command.get_action(data)?),
      LookAround(command) => Ok(command.get_action(data)?),
      LookAtEntity(command) => Ok(command.get_action(data)?),
      LookDirection(command) => Ok(command.get_action(data)?),
      Quit(command) => Ok(command.get_action(data)?),
    }
  }
}
