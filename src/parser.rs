extern crate anyhow;

use anyhow::Error;
use hornvale::ecs::component::*;
use hornvale::ecs::resource::*;
use hornvale::input::Input as InputSystem;
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

impl<'a> System<'a> for InputProcessor {
  type SystemData = Data<'a>;

  /// Run system.
  fn run(&mut self, _data: Self::SystemData) {
    let input = self.input.as_ref().unwrap().clone();
    self.output = self.input_system.interpret(&input).unwrap();
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
    ip.input = Some(line);
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
