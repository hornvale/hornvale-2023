extern crate anyhow;

use anyhow::Error;
use hornvale::ecs::component::*;
use hornvale::ecs::entity::{EntityId, PlayerId};
use hornvale::ecs::resource::*;
use hornvale::input::Input as InputSystem;
use hornvale::input::ParserData;
use rand_seeder::SipHasher;
use specs::prelude::*;
use std::env::args;
use std::io::{self, BufRead, Write as IoWrite};
use std::process::exit;

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
  fn get_player_id(&self) -> Result<PlayerId, Error> {
    Ok(PlayerId(3))
  }
  /// Retrieve a list of nouns.
  fn get_nouns(&self) -> Result<Vec<(String, EntityId)>, Error> {
    Ok(vec![
      ("cow".to_string(), EntityId(1)),
      ("echo".to_string(), EntityId(2)),
      ("priest".to_string(), EntityId(3)),
      ("rock".to_string(), EntityId(4)),
    ])
  }
  /// Retrieve a list of adjectives.
  fn get_adjectives(&self) -> Result<Vec<String>, Error> {
    Ok(vec![
      "brown".to_string(),
      "green".to_string(),
      "pale".to_string(),
      "arrogant".to_string(),
      "granite".to_string(),
    ])
  }
  /// Retrieve a list of genitives.
  fn get_genitives(&self) -> Result<Vec<String>, Error> {
    Ok(vec!["priest's".to_string(), "cow's".to_string()])
  }
}

impl<'a> System<'a> for InputProcessor {
  type SystemData = Data<'a>;

  /// Run system.
  fn run(&mut self, data: Self::SystemData) {
    let input = self.input.as_ref().unwrap().clone();
    self.output = match self.input_system.interpret(&input, &data) {
      Ok((_command, string_opt)) => Some(string_opt.unwrap_or_else(|| "OK".to_string())),
      Err(error) => Some(error.to_string()),
    };
  }
}

pub fn repl<R, W>(mut input: R, mut output: W) -> Result<(), Error>
where
  R: BufRead,
  W: IoWrite,
{
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
  let mut ip = InputProcessor {
    input: None,
    output: None,
    input_system: InputSystem::default(),
  };
  loop {
    write!(&mut output, "> ")?;
    output.flush()?;
    let mut line = String::new();
    input.read_line(&mut line)?;
    if line.is_empty() {
      break;
    }
    ip.input = Some(line.trim().to_string());
    ip.run_now(&ecs);
    ecs.maintain();
    ip.input = None;
    writeln!(&mut output, "{}", ip.output.unwrap_or_default())?;
    ip.output = None;
  }
  Ok(())
}

fn main() -> Result<(), Error> {
  use pretty_env_logger::env_logger::builder;
  let _ = builder().is_test(true).try_init();
  match args().count() {
    1 => {
      let stdio = io::stdin();
      let input = stdio.lock();
      let output = io::stdout();
      repl(input, output)
    },
    _ => exit(-1),
  }
}
