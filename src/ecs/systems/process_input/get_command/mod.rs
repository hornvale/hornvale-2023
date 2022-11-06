use anyhow::Error as AnyError;
use std::str::FromStr;

use crate::command::Command;
use crate::map::Direction;

use super::*;

impl<'a> ProcessInput {
  /// Get the command corresponding to this input.
  pub fn get_command(&mut self, input: &str, data: &mut ProcessInputData<'a>) -> Result<Command, AnyError> {
    let original_input = (*input).to_owned();
    let words: Vec<&str> = input.split_whitespace().collect();
    let first: String = words.first().unwrap_or(&"").to_string();
    let second: String = words.get(1).unwrap_or(&"").to_string();
    use Command::*;
    let player_id = data.player_resource.0.unwrap();
    // let player = data.entities.entity(player_id.0);
    let result = match (first.as_str(), second.as_str()) {
      ("echo", _) => Ok(Echo {
        player_id,
        string: words[1..].join(" "),
        original_input,
      }),
      ("eval", _) => Ok(Eval {
        player_id,
        string: words[1..].join(" "),
        original_input,
      }),
      ("look" | "l", direction) if Direction::from_str(direction).is_ok() => Ok(LookDirection {
        player_id,
        direction: Direction::from_str(direction).unwrap(),
        original_input,
      }),
      ("look" | "l", _) => {
        if let Ok(object) = self.match_visible_object(&words[1..].join(" "), data) {
          Ok(LookAtObject {
            player_id,
            object_id: ObjectId(object.id()),
            original_input,
          })
        } else {
          Err(anyhow!("Unexpected combination!"))
        }
      },
      (direction, _) | ("move" | "go", direction) if Direction::from_str(direction).is_ok() => Ok(MoveDirection {
        player_id,
        direction: Direction::from_str(direction).unwrap(),
        original_input,
      }),
      ("quit", _) => Ok(Quit {
        player_id,
        original_input,
      }),
      (&_, _) => Err(anyhow!("Unexpected combination!")),
    };
    result
  }
}