use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::ecs::component::*;
use crate::ecs::event::*;
use crate::ecs::resource::*;

/// The `AllData` type.
///
/// This represents all data available in the ECS world at any time.
///
/// Obviously, this should be used as infrequently as possible, but it is
/// necessary in some scenarios, particularly processing actions and effects.
#[derive(SystemData)]
pub struct AllData<'data> {
  pub entities: Entities<'data>,
  pub camera_resource: Read<'data, CameraResource>,
  pub player_resource: Read<'data, PlayerResource>,
  pub random_resource: Read<'data, RandomResource>,
  pub tile_map_resource: Write<'data, TileMapResource>,
  pub action_event_channel: Write<'data, EventChannel<ActionEvent>>,
  pub effect_event_channel: Write<'data, EventChannel<EffectEvent>>,
  pub output_event_channel: Write<'data, EventChannel<OutputEvent>>,
  pub has_brief_description: ReadStorage<'data, HasBriefDescription>,
  pub has_gender: ReadStorage<'data, HasGender>,
  pub has_name: ReadStorage<'data, HasName>,
  pub has_passages: ReadStorage<'data, HasPassages>,
  pub is_a_player: ReadStorage<'data, IsAPlayer>,
  pub is_an_actor: ReadStorage<'data, IsAnActor>,
  pub is_an_object: ReadStorage<'data, IsAnObject>,
  pub is_in_room: WriteStorage<'data, IsInRoom>,
}
