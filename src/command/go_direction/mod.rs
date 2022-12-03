use crate::action::*;
use crate::command::{Command, Commandable};
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data;
use crate::input::{ParserData, Token, TokenType};
use crate::map::Direction;
use anyhow::Error as AnyError;
use std::str::FromStr;

/// The `GoDirection` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GoDirection {
  pub player_id: PlayerId,
  pub direction: Direction,
  pub original_input: String,
}

impl GoDirection {
  /// Create a command based on the parser tokens and the passed data.
  pub fn from_data(
    original_input: String,
    _string: String,
    tokens: Vec<Token<'_>>,
    data: &impl ParserData,
  ) -> Result<Command, AnyError> {
    let second = tokens.get(1);
    let player_id = data.get_player_id()?;
    match second {
      Some(second) => match second.r#type {
        TokenType::Direction => Ok(create_command!(Self {
          player_id,
          direction: Direction::from_str(second.lexeme).unwrap(),
          original_input,
        })),
        _ => Err(anyhow!(
          "Couldn't find a good match for the second token of {:#?}",
          tokens
        )),
      },
      None => Err(anyhow!(
        "Couldn't find a good match for the second token of {:#?}",
        tokens
      )),
    }
  }
}

impl Commandable for GoDirection {
  fn get_action(&self, _data: &mut Data) -> Result<Option<Action>, AnyError> {
    Ok(Some(create_action!(GoDirectionAction {
      entity_id: self.player_id.into(),
      direction: self.direction,
    })))
  }
}
