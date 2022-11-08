use crate::action::Action;
use crate::map::Direction;

use super::*;

impl<'a> CreateMap {
  /// Create a "trivial maze" demo.
  pub fn create_trivial_maze(&mut self, data: &mut CreateMapData<'a>) {
    let mut rooms = Vec::new();
    let width = 4;
    let height = 4;
    let total = width * height;
    for i in 0..total {
      let room_id = create_room!(
        data,
        format!("Room {}", i),
        format!("This is room {}, or {}/{}", i, i + 1, total)
      );
      rooms.push(room_id);
    }
    if let Some(player_id) = data.player_resource.0 {
      set_current_room!(data, get_entity!(data, player_id), rooms[0]);
      data.action_event_channel.single_write(ActionEvent {
        action: Action::LookAround {
          entity_id: player_id.into(),
        },
      });
    }
    for (index, room) in rooms.iter().enumerate() {
      // We don't need to create passages to the west or north, since they
      // should be created by the room to the west or north respectively.
      let is_on_right_side = index % width == width - 1;
      let is_on_bottom_side = total - index - 1 < width;
      if !is_on_right_side {
        create_passage!(data, *room, rooms[index + 1], &Direction::East, true);
      }
      if !is_on_bottom_side {
        create_passage!(data, *room, rooms[index + width], &Direction::South, true);
      }
    }
  }
}
