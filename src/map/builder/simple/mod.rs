use crate::components::has_description::HasDescription;
use crate::components::has_passages::HasPassages;
use crate::direction::Direction;
use crate::map::Map;
use crate::passage::destination::Destination;
use crate::passage::Passage;
use crate::room::id::Id as RoomId;
use crate::room::Room;

/// The `Simple` map builder.
#[derive(Clone, Copy, Debug)]
pub struct Simple {}

impl Simple {
  pub fn build(&self) -> Map {
    let mut map = Map::new();
    let mut rooms = Vec::new();
    let width = 4;
    let height = 4;
    let total = width * height;
    for i in 0..total {
      let room = Room {
        has_description: Some(HasDescription {
          brief: format!("This is room {}, or {}/{}", i, i + 1, total),
          initial: None,
        }),
        ..Room::default()
      };
      rooms.push(room);
    }
    let room_ids: Vec<RoomId> = rooms.iter().map(|room| room.id.clone()).collect();
    for (index, mut room) in rooms.iter_mut().enumerate() {
      let mut has_passages = HasPassages::default();
      let is_on_left_side = index % width == 0;
      let is_on_right_side = index % width == width - 1;
      let is_on_top_side = index < width;
      let is_on_bottom_side = total - index - 1 < width;
      if !is_on_left_side {
        let neighbor = index - 1;
        let room_id = room_ids[neighbor].clone();
        has_passages.west = Some(Passage {
          direction: Direction::West,
          destination: Destination::Room(room_id.clone()),
        });
      }
      if !is_on_right_side {
        let neighbor = index + 1;
        let room_id = room_ids[neighbor].clone();
        has_passages.east = Some(Passage {
          direction: Direction::East,
          destination: Destination::Room(room_id.clone()),
        });
      }
      if !is_on_top_side {
        let neighbor = index - width;
        let room_id = room_ids[neighbor].clone();
        has_passages.north = Some(Passage {
          direction: Direction::North,
          destination: Destination::Room(room_id.clone()),
        });
      }
      if !is_on_bottom_side {
        let neighbor = index + width;
        let room_id = room_ids[neighbor].clone();
        has_passages.south = Some(Passage {
          direction: Direction::South,
          destination: Destination::Room(room_id.clone()),
        });
      }
      room.has_passages = Some(has_passages);
    }
    map.spawn_room_id = Some(rooms[0].id.clone());
    map.rooms = rooms.into_iter().map(|room| (room.id.clone(), room)).collect();
    map
  }
}
