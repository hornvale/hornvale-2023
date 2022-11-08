use rand_seeder::SipHasher;
use rustyline_async::Readline;
use specs::prelude::*;

pub mod input;
pub use input::Input as InputResource;

pub mod output;
pub use output::Output as OutputResource;

pub mod player;
pub use player::Player as PlayerResource;

pub mod random;
pub use random::Random as RandomResource;

pub mod spawn_room;
pub use spawn_room::SpawnRoom as SpawnRoomResource;

pub mod tick;
pub use tick::Tick as TickResource;

pub fn insert_resources(ecs: &mut World, seed: &str) {
  let (input, stdout) = Readline::new("> ".to_owned()).unwrap();
  ecs.insert(InputResource(Some(input)));
  ecs.insert(OutputResource(Some(stdout)));
  ecs.insert(PlayerResource(None));
  let rng = SipHasher::from(seed).into_rng();
  ecs.insert(RandomResource(rng));
  ecs.insert(SpawnRoomResource(None));
  ecs.insert(TickResource(0));
}
