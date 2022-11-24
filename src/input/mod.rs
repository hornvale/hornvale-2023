use anyhow::Error;

/// The `Input` object.
#[derive(Clone, Copy, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub struct Input {
  pub had_error: bool,
}

impl Input {
  pub fn interpret(&mut self, _input: &str) -> Result<Option<String>, Error> {
    Ok(Some("OK".to_string()))
  }
}

#[cfg(test)]
pub mod test {

  use super::Input as InputSystem;
  use super::*;
  use crate::ecs::component::*;
  use crate::ecs::resource::*;
  use crate::test::*;
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

  impl<'a> System<'a> for InputProcessor {
    type SystemData = Data<'a>;

    /// Run system.
    fn run(&mut self, mut _data: Self::SystemData) {
      let input = self.input.as_ref().unwrap().clone();
      self.output = self.input_system.interpret(&input).unwrap();
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
