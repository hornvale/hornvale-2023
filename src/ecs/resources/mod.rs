use rustyline_async::Readline;
use specs::prelude::*;
use std::collections::VecDeque;

pub mod input;
pub use input::Input as InputResource;

pub mod messages;
pub use messages::Messages as MessagesResource;

pub mod output;
pub use output::Output as OutputResource;

pub mod player;
pub use player::Player as PlayerResource;

pub mod spawn_room;
pub use spawn_room::SpawnRoom as SpawnRoomResource;

pub mod tick;
pub use tick::Tick as TickResource;

pub fn insert_resources(ecs: &mut World) {
  let (input, stdout) = Readline::new("> ".to_owned()).unwrap();
  ecs.insert(InputResource(Some(input)));
  ecs.insert(MessagesResource(VecDeque::new()));
  ecs.insert(OutputResource(Some(stdout)));
  ecs.insert(PlayerResource(None));
  ecs.insert(SpawnRoomResource(None));
  ecs.insert(TickResource(0));
}
