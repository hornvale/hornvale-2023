use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::ecs::component::*;
use crate::ecs::entity::EntityId;
use crate::ecs::event::*;
use crate::ecs::resource::*;

pub struct AiProcessor {}

impl AiProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub camera_resource: Read<'a, CameraResource>,
  pub player_resource: Read<'a, PlayerResource>,
  pub random_resource: Write<'a, RandomResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub effect_event_channel: Write<'a, EventChannel<EffectEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_ai: ReadStorage<'a, HasAi>,
  pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
  pub has_gender: ReadStorage<'a, HasGender>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub has_state: ReadStorage<'a, HasState>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_actor: ReadStorage<'a, IsAnActor>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
  pub lazy_updater: Read<'a, LazyUpdate>,
}

impl<'a> System<'a> for AiProcessor {
  type SystemData = Data<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let entity_ais = (&data.entities, &data.has_ai)
      .join()
      .map(|(entity, has_ai)| (entity, has_ai.clone()))
      .collect::<Vec<(Entity, HasAi)>>();

    for (entity, has_ai) in entity_ais {
      let ai = &has_ai.0;
      match ai.get_action(EntityId(entity.id()), &mut data) {
        Ok(Some(action)) => {
          info!("Calculated intradigetic action {:?} for ai {:?}...", action, ai);
          write_action_event!(data, action);
        },
        Ok(None) => {
          info!("Processed extradiegetic ai {:?}...", ai);
        },
        Err(error) => write_output_event!(data, format!("encountered an error ({})", error)),
      }
    }
  }
}
