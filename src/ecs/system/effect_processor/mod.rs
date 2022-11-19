use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::ecs::component::*;
use crate::ecs::event::*;
use crate::resource::*;

pub struct EffectProcessor {
  pub reader_id: ReaderId<EffectEvent>,
}

impl EffectProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Read<'a, PlayerResource>,
  pub camera_resource: Read<'a, CameraResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub effect_event_channel: Write<'a, EventChannel<EffectEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_ai: ReadStorage<'a, HasAi>,
  pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub has_state: WriteStorage<'a, HasState>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_actor: ReadStorage<'a, IsAnActor>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for EffectProcessor {
  type SystemData = Data<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .effect_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<EffectEvent>>();
    let event_count = events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} effect event(s)...", event_count);
    for event in events.iter() {
      debug!("Processing next effect event, {:?}", event);
      let EffectEvent { effect } = event;
      match effect.process(&mut data) {
        Ok(()) => {},
        Err(error) => write_output_event!(data, format!("encountered an error ({})", error)),
      }
    }
  }
}
