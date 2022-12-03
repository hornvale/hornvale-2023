use crate::action::Action;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use crate::input::ParserData;
use crate::input::{Token, TokenType};
use anyhow::Error as AnyError;
use std::sync::Arc;

pub mod _trait;
pub use _trait::*;
pub mod echo;
pub use echo::Echo as EchoCommand;
pub mod eval;
pub use eval::Eval as EvalCommand;
pub mod go_direction;
pub use go_direction::GoDirection as GoDirectionCommand;
pub mod idle;
pub use idle::Idle as IdleCommand;
pub mod look;
pub use look::*;
pub mod order;
pub use order::Order as OrderCommand;
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
#[derive(Clone, Debug)]
pub struct Command(pub Arc<dyn Commandable>);

impl Command {
  /// Create a command based on the parser tokens and the passed data.
  pub fn from_data(
    original_input: String,
    string: String,
    tokens: Vec<Token<'_>>,
    data: &impl ParserData,
  ) -> Result<Command, AnyError> {
    let player_id = data.get_player_id()?;
    if let Some(first) = tokens.first() {
      match first.r#type {
        TokenType::Echo => Ok(create_command!(EchoCommand {
          player_id,
          string,
          original_input,
        })),
        TokenType::Eval => Ok(create_command!(EvalCommand {
          player_id,
          string,
          original_input,
        })),
        TokenType::Go => Ok(create_command!(GoDirectionCommand::from_data(
          original_input,
          string,
          tokens,
          data
        )?)),
        TokenType::Look => Ok(create_command!(LookCommandFactory::from_data(
          original_input,
          string,
          tokens,
          data
        )?)),
        TokenType::Quit => Ok(create_command!(QuitCommand {
          player_id,
          original_input,
        })),
        _ => Err(anyhow!("Couldn't match first token: {:#?}", tokens)),
      }
    } else {
      bail!("Couldn't get first token.")
    }
  }
}

impl Commandable for Command {
  /// Retrieve an action for this command, or evaluate it.
  ///
  /// Commands come in two forms: extradiegetic and intradiegetic.
  ///
  /// Extradiegetic or out-of-character/OOC commands operate outside the game
  /// world.  They generally perform some operation on game state and return
  /// directly.
  ///
  /// Intradiegetic or in-character/IC commands are translated to an Action
  /// object that represents the in-game action to take.
  ///
  /// Thus extradiegetic commands will return None here, and intradiegetic
  /// commands will return Some(action).
  fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, AnyError> {
    (*self.0).get_action(data)
  }
}
