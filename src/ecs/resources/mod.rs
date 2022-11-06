use specs::prelude::*;
use std::collections::VecDeque;

pub mod messages;
pub use messages::Messages as MessagesResource;

pub mod player;
pub use player::Player as PlayerResource;

pub mod spawn_room;
pub use spawn_room::SpawnRoom as SpawnRoomResource;

pub mod tick;
pub use tick::Tick as TickResource;

pub fn insert_resources(ecs: &mut World) {
  ecs.insert(MessagesResource(VecDeque::new()));
  ecs.insert(PlayerResource(None));
  ecs.insert(SpawnRoomResource(None));
  ecs.insert(TickResource(0));
}
