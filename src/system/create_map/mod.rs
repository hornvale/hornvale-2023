use rand::prelude::*;
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::component::*;
use crate::entity::*;
use crate::event::*;
use crate::resource::*;

mod compass_demo;
mod trivial_maze;

pub struct CreateMap {}

#[derive(SystemData)]
pub struct CreateMapData<'a> {
  pub entities: Entities<'a>,
  pub camera_resource: Write<'a, CameraResource>,
  pub player_resource: Write<'a, PlayerResource>,
  pub random_resource: Write<'a, RandomResource>,
  pub spawn_room_resource: Write<'a, SpawnRoomResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub has_brief_description: WriteStorage<'a, HasBriefDescription>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
  pub has_name: WriteStorage<'a, HasName>,
  pub has_passages: WriteStorage<'a, HasPassages>,
  pub is_a_room: WriteStorage<'a, IsARoom>,
  pub is_an_actor: WriteStorage<'a, IsAnActor>,
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
