use specs::prelude::*;

use super::super::components::*;
use super::super::entity::RoomId;
use super::super::resources::*;

use crate::map::Direction;

pub struct CreateMap {}

#[derive(SystemData)]
pub struct CreateMapData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub spawn_room_resource: Write<'a, SpawnRoomResource>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_passages: WriteStorage<'a, HasPassages>,
  pub has_name: WriteStorage<'a, HasName>,
  pub is_a_room: WriteStorage<'a, IsARoom>,
  pub is_an_object: WriteStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for CreateMap {
  type SystemData = CreateMapData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    let spawn_room = create_room!(data, "Spawn Room", "Dark olive trees crowd in on all sides, the air steams with the mist of a warm recent rain, midges hang in the air.");
    if let Some(player_id) = data.player_resource.0 {
      let player = data.entities.entity(player_id.0);
      data
        .is_in_room
        .insert(player, IsInRoom(RoomId(spawn_room.id())))
        .expect("Unable to insert is-in-room for entity!");
    }
    let _mushroom = create_object!(
      data,
      "Mushroom",
      "A speckled mushroom grows out of the sodden earth, on a long stalk.",
      RoomId(spawn_room.id())
    );
    let ne_room = create_room!(data, "Northeast Room", "This is the Northeastern Room.");
    let n_room = create_room!(data, "North Room", "This is the Northern Room.");
    let nw_room = create_room!(data, "Northwest Room", "This is the Northwestern Room.");
    let e_room = create_room!(data, "East Room", "This is the Eastern Room.");
    let w_room = create_room!(data, "West Room", "This is the Western Room.");
    let se_room = create_room!(data, "Southeast Room", "This is the Southeastern Room.");
    let s_room = create_room!(data, "South Room", "This is the Southern Room.");
    let sw_room = create_room!(data, "Southwest Room", "This is the Southwestern Room.");
    create_passage!(data, spawn_room, ne_room, &Direction::Northeast, true);
    create_passage!(data, spawn_room, n_room, &Direction::North, true);
    create_passage!(data, n_room, ne_room, &Direction::East, true);
    create_passage!(data, spawn_room, nw_room, &Direction::Northwest, true);
    create_passage!(data, n_room, nw_room, &Direction::West, true);
    create_passage!(data, spawn_room, e_room, &Direction::East, true);
    create_passage!(data, spawn_room, w_room, &Direction::West, true);
    create_passage!(data, spawn_room, se_room, &Direction::Southeast, true);
    create_passage!(data, spawn_room, s_room, &Direction::South, true);
    create_passage!(data, spawn_room, sw_room, &Direction::Southwest, true);
    data.spawn_room_resource.0 = Some(RoomId(spawn_room.id()));
  }
}

/*
use specs::prelude::*;

use crate::ecs::components::has_description::HasDescription;
use crate::ecs::components::has_passages::HasPassages;
use crate::map::Direction;
use crate::map::Passage;
use crate::map::PassageDestination;
use crate::ecs::entity::RoomId;

/// The `Simple` map builder.
#[derive(Clone, Copy, Debug)]
pub struct Simple {}

impl Simple {
  pub fn build(&self, ecs: &mut World) {
    let mut room_ids = Vec::new();
    let width = 4;
    let height = 4;
    let total = width * height;
    for i in 0..total {
      let room_id = ecs
        .create_entity()
        .with(HasDescription {
          brief: format!("This is room {}, or {}/{}", i, i + 1, total),
          initial: None,
        })
        .build();
      room_ids.push(room_id);
    }
    for (index, mut room_id) in room_ids.iter().enumerate() {
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
          destination: PassageDestination::Room(room_id.clone()),
        });
      }
      if !is_on_right_side {
        let neighbor = index + 1;
        let room_id = room_ids[neighbor].clone();
        has_passages.east = Some(Passage {
          direction: Direction::East,
          destination: PassageDestination::Room(room_id.clone()),
        });
      }
      if !is_on_top_side {
        let neighbor = index - width;
        let room_id = room_ids[neighbor].clone();
        has_passages.north = Some(Passage {
          direction: Direction::North,
          destination: PassageDestination::Room(room_id.clone()),
        });
      }
      if !is_on_bottom_side {
        let neighbor = index + width;
        let room_id = room_ids[neighbor].clone();
        has_passages.south = Some(Passage {
          direction: Direction::South,
          destination: PassageDestination::Room(room_id.clone()),
        });
      }
      room.has_passages = Some(has_passages);
    }
    map.spawn_room_id = Some(rooms[0].id.clone());
    map.rooms = rooms.into_iter().map(|room| (room.id.clone(), room)).collect();
  }
}
*/
