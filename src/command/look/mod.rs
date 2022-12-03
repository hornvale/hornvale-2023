use crate::command::Command;
use crate::input::{ParserData, Token, TokenType};
use crate::map::Direction;
use anyhow::Error as AnyError;
use std::str::FromStr;
pub mod around;
pub use around::LookAround as LookAroundCommand;
pub mod at_entity;
pub use at_entity::LookAtEntity as LookAtEntityCommand;
pub mod direction;
pub use direction::LookDirection as LookDirectionCommand;

/// The `LookCommandFactory` type.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookCommandFactory {}

impl LookCommandFactory {
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
        TokenType::Direction => Ok(create_command!(LookDirectionCommand {
          player_id,
          direction: Direction::from_str(second.lexeme).unwrap(),
          original_input,
        })),
        _ => {
          println!("Second: {:#?}", second);
          if second.entity_id.is_some() {
            let target_entity_id = second.entity_id.unwrap();
            Ok(create_command!(LookAtEntityCommand {
              player_id,
              target_entity_id,
              original_input,
            }))
          } else {
            Ok(create_command!(LookAroundCommand {
              player_id,
              original_input,
            }))
          }
        },
      },
      None => Ok(create_command!(LookAroundCommand {
        player_id,
        original_input,
      })),
    }
  }
}
