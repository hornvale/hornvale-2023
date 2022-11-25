use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::ecs::event::*;
use crate::ecs::resource::*;
use crate::input::{Input, ParserData};
use anyhow::Error as AnyError;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

pub struct InputProcessor {
  pub reader_id: ReaderId<InputEvent>,
}

impl InputProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Read<'a, PlayerResource>,
  pub output_resource: Write<'a, OutputResource>,
  pub command_event_channel: Write<'a, EventChannel<CommandEvent>>,
  pub input_event_channel: Read<'a, EventChannel<InputEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: ReadStorage<'a, IsInRoom>,
}

impl<'a> ParserData for Data<'a> {
  /// Retrieve the player ID.
  fn get_player_id(&self) -> Result<PlayerId, AnyError> {
    Ok(get_player_id!(self))
  }
  /// Retrieve a list of nouns.
  fn get_nouns(&self) -> Result<Vec<(String, EntityId)>, AnyError> {
    let player_id = get_player_id!(self);
    let player = get_entity!(self, player_id);
    if let Some(current_room) = get_current_room_id!(self, player) {
      let result = (
        &self.entities,
        &self.is_in_room,
        &self.has_name,
        &self.has_brief_description,
        !&self.is_a_player,
      )
        .join()
        .filter(|(_entity, is_in_room, _has_name, _has_brief_description, _)| is_in_room.0 == current_room)
        .map(|(entity, _is_in_room, has_name, _has_brief_description, _)| {
          (has_name.0.to_lowercase(), EntityId(entity.id()))
        })
        .collect::<Vec<(String, EntityId)>>();
      return Ok(result);
    }
    Err(anyhow!("Not found"))
  }
  /// Retrieve a list of genitives.
  fn get_genitives(&self) -> Result<Vec<String>, AnyError> {
    Ok(vec![])
  }
  /// Retrieve a list of adjectives.
  fn get_adjectives(&self) -> Result<Vec<String>, AnyError> {
    Ok(vec![])
  }
}

impl<'a> System<'a> for InputProcessor {
  type SystemData = Data<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    let input_events = data
      .input_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<InputEvent>>();
    let event_count = input_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} input event(s)...", event_count);
    let input_manager = Input::default();
    for event in input_events.iter() {
      let input_string = &event.input;
      match input_manager.interpret(input_string, &data) {
        Ok((command, string_opt)) => {
          write_command_event!(data, command);
          if let Some(string) = string_opt {
            write_output_event!(data, string);
          }
        },
        Err(error) => write_output_event!(data, format!("Something was screwed up about {:?}: {}", event, error)),
      }
    }
  }
}
