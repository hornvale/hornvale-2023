use super::super::components::*;
use super::super::entity::*;
use super::super::event_channels::*;
use super::super::resources::*;
use crate::action::Action;
use crate::map::Direction;
use rand::prelude::*;
use specs::prelude::*;
use specs::shrev::EventChannel;

pub struct Experiment {}

#[derive(SystemData)]
pub struct ExperimentData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub spawn_room_resource: Write<'a, SpawnRoomResource>,
  pub random_resource: Write<'a, RandomResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_name: WriteStorage<'a, HasName>,
  pub has_passages: WriteStorage<'a, HasPassages>,
  pub is_a_player: WriteStorage<'a, IsAPlayer>,
  pub is_a_room: WriteStorage<'a, IsARoom>,
  pub is_an_actor: WriteStorage<'a, IsAnActor>,
  pub is_an_object: WriteStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for Experiment {
  type SystemData = ExperimentData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    for (entity, _, _) in (&data.entities, &data.is_an_actor, !&data.is_a_player).join() {
      let direction: Direction = data.random_resource.0.gen();
      let action = Action::GoDirection {
        entity_id: EntityId(entity.id()),
        direction,
      };
      data.action_event_channel.single_write(ActionEvent { action });
    }
  }
}
