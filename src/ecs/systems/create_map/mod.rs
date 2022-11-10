use rand::prelude::*;
use specs::prelude::*;
use specs::shrev::EventChannel;

use super::super::components::*;
use super::super::entity::*;
use super::super::event_channels::*;
use super::super::resources::*;

mod compass_demo;
mod trivial_maze;

pub struct CreateMap {}

#[derive(SystemData)]
pub struct CreateMapData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub spawn_room_resource: Write<'a, SpawnRoomResource>,
  pub random_resource: Write<'a, RandomResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_passages: WriteStorage<'a, HasPassages>,
  pub has_name: WriteStorage<'a, HasName>,
  pub is_a_being: WriteStorage<'a, IsABeing>,
  pub is_a_room: WriteStorage<'a, IsARoom>,
  pub is_an_object: WriteStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for CreateMap {
  type SystemData = CreateMapData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    let rng = &mut data.random_resource.0;
    if rng.gen::<bool>() {
      self.create_compass_demo(&mut data);
    } else {
      self.create_trivial_maze(&mut data);
    }
  }
}
