use anyhow::Error as AnyError;
use std::str::FromStr;

use crate::command::*;
use crate::map::Direction;

use super::*;

impl<'a> InputProcessor {
  /// Get the command corresponding to this input.
  pub fn get_command(&mut self, input: &str, data: &mut Data<'a>) -> Result<Command, AnyError> {
    let original_input = (*input).to_owned();
    let words: Vec<&str> = input.split_whitespace().collect();
    let first: String = words.first().unwrap_or(&"").to_string();
    let second: String = words.get(1).unwrap_or(&"").to_string();
    use Command::*;
    let player_id = data.player_resource.0.unwrap();
    let result = match (first.as_str(), second.as_str()) {
      ("echo", _) => {
        debug!("Matched command Echo");
        Ok(Echo(EchoCommand {
          player_id,
          string: words[1..].join(" "),
          original_input,
        }))
      },
      ("eval", _) => {
        debug!("Matched command Eval");
        Ok(Eval(EvalCommand {
          player_id,
          string: words[1..].join(" "),
          original_input,
        }))
      },
      ("look" | "l", direction) if Direction::from_str(direction).is_ok() => {
        debug!("Matched command LookDirection");
        Ok(LookDirection(LookDirectionCommand {
          player_id,
          direction: Direction::from_str(direction).unwrap(),
          original_input,
        }))
      },
      ("look" | "l", _) => {
        if let Ok(target_entity) = self.match_visible_entity(&words[1..].join(" "), data) {
          debug!("Matched command LookAtObject");
          Ok(LookAtEntity(LookAtEntityCommand {
            player_id,
            target_entity_id: EntityId(target_entity.id()),
            original_input,
          }))
        } else {
          debug!("Matched command LookAround");
          Ok(LookAround(LookAroundCommand {
            player_id,
            original_input,
          }))
        }
      },
      (direction, _) | ("move" | "go", direction) if Direction::from_str(direction).is_ok() => {
        debug!("Matched command GoDirection");
        Ok(GoDirection(GoDirectionCommand {
          player_id,
          direction: Direction::from_str(direction).unwrap(),
          original_input,
        }))
      },
      ("quit", _) => {
        debug!("Matched command Quit");
        Ok(Quit(QuitCommand {
          player_id,
          original_input,
        }))
      },
      (&_, _) => Err(anyhow!("Unexpected combination!")),
    };
    result
  }
}
