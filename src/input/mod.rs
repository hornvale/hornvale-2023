use crate::command::Command;
use anyhow::Error;
pub mod _trait;
pub use _trait::parser_data::ParserData;
pub mod parser;
pub use parser::Parser;
pub mod scanner;
pub use scanner::Scanner;
pub mod token;
pub use token::{Token, TokenType};

/// The `Input` type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub struct Input {}

impl Input {
  pub fn interpret(&self, input: &str, data: &impl ParserData) -> Result<(Command, Option<String>), Error> {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens()?;
    let mut parser = Parser::new(scanner.tokens, input);
    let command = parser.parse(data)?;
    Ok((command, Some("OK".to_string())))
  }
}

#[cfg(test)]
pub mod test {

  use super::Input as InputSystem;
  use super::*;
  use crate::ecs::component::*;
  use crate::ecs::entity::{EntityId, PlayerId};
  use crate::ecs::resource::*;
  use crate::test::*;
  use anyhow::Error as AnyError;
  use rand_seeder::SipHasher;
  use specs::prelude::*;

  pub struct InputProcessor {
    pub input: Option<String>,
    pub output: Option<String>,
    pub input_system: InputSystem,
  }

  impl InputProcessor {}

  #[derive(SystemData)]
  pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
    pub has_name: ReadStorage<'a, HasName>,
    pub has_passages: ReadStorage<'a, HasPassages>,
    pub is_a_player: ReadStorage<'a, IsAPlayer>,
    pub is_an_object: ReadStorage<'a, IsAnObject>,
    pub is_in_room: ReadStorage<'a, IsInRoom>,
  }

  impl<'a> ParserData for Data<'a> {
    /// Retrieve the player ID.
    fn get_player_id(&self) -> Result<PlayerId, AnyError> {
      Ok(PlayerId(3))
    }
    /// Retrieve a list of nouns.
    fn get_nouns(&self) -> Result<Vec<(String, EntityId)>, AnyError> {
      Ok(vec![])
    }
    /// Retrieve a list of genitives.
    fn get_genitives(&self) -> Result<Vec<String>, AnyError> {
      Ok(vec![])
    }
    /// Retrieve a list of adjectives.
    fn get_adjectives(&self) -> Result<Vec<String>, AnyError> {
      Ok(vec![])
    }
  }

  impl<'a> System<'a> for InputProcessor {
    type SystemData = Data<'a>;

    /// Run system.
    fn run(&mut self, data: Self::SystemData) {
      let input = self.input.as_ref().unwrap().clone();
      self.output = self.input_system.interpret(&input, &data).unwrap().1;
    }
  }

  #[test]
  pub fn test_input() {
    init();
    let mut ecs = World::new();
    let seed = "goat boy";
    let rng = SipHasher::from(seed).into_rng();
    ecs.insert(CameraResource(None));
    ecs.insert(PlayerResource(None));
    ecs.insert(RandomResource(rng));
    ecs.insert(SpawnRoomResource(None));
    ecs.insert(TickResource(0));
    ecs.insert(TileMapResource(None));
    register_components(&mut ecs);
    let _input = Input::default();
  }
}
