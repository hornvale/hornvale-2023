use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::component::*;
use crate::event::*;
use crate::resource::*;

pub struct ActionProcessor {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ActionProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub camera_resource: Read<'a, CameraResource>,
  pub player_resource: Read<'a, PlayerResource>,
  pub random_resource: Read<'a, RandomResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub effect_event_channel: Write<'a, EventChannel<EffectEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
  pub has_gender: ReadStorage<'a, HasGender>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_actor: ReadStorage<'a, IsAnActor>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for ActionProcessor {
  type SystemData = Data<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .action_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<ActionEvent>>();
    let event_count = events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} action event(s)...", event_count);
    for event in events.iter() {
      debug!("Processing next action event, {:?}", event);
      let ActionEvent { action } = event;
      match action.execute(&mut data) {
        Ok(()) => {},
        Err(error) => error!("{:#?}", error),
      }
    }
  }
}
