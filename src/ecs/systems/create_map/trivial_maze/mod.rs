use rand::prelude::*;

use crate::action::Action;
use crate::map::{Direction, Tile, TileMap};

use super::*;

impl<'a> CreateMap {
  /// Create a "trivial maze" demo.
  pub fn create_trivial_maze(&mut self, data: &mut CreateMapData<'a>) {
    let mut rooms = Vec::new();
    let width = 30;
    let height = 10;
    let total = width * height;
    let mut tilemap = TileMap::new(width, height);
    for i in 0..total {
      let room_id = create_room!(
        data,
        format!("Room {}", i + 1),
        format!("This is room {}/{}", i + 1, total)
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
      let room_id = RoomId(room.id());
      let x = index % width;
      let y = index / width;
      let is_on_right_side = x == width - 1;
      let is_on_bottom_side = y == height - 1;
      let t_index = tilemap.get_index(x, y);
      tilemap.map[t_index] = Tile::Floor;
      tilemap.set_room_id(room_id, x, y);
      let east: bool = data.random_resource.0.gen();
      if !is_on_right_side && (east || is_on_bottom_side) {
        create_passage!(data, *room, rooms[index + 1], &Direction::East, true);
        tilemap.map[t_index + 1] = Tile::Floor;
      } else if !is_on_bottom_side {
        create_passage!(data, *room, rooms[index + width], &Direction::South, true);
        tilemap.map[t_index + tilemap.width] = Tile::Floor;
      }
    }
    data.tile_map_resource.0 = Some(tilemap);
  }
}
