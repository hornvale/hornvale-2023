use super::super::{Direction, Tile, TileMap};
use crate::action::Action;
use crate::action::LookAroundAction;
use crate::entity::RoomId;
use crate::system::create_map::CreateMapData as Data;
use rand::prelude::*;

pub struct TrivialMaze {}

impl<'a> TrivialMaze {
  /// Create a "trivial maze" demo.
  pub fn build(&mut self, data: &mut Data<'a>) {
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
      is_in_room!(data, get_entity!(data, player_id), RoomId(rooms[0].id()));
      write_action_event!(
        data,
        Action::LookAround(LookAroundAction {
          entity_id: player_id.into(),
        })
      );
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
