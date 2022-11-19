use super::super::Direction;
use crate::effect::*;
use crate::entity::RoomId;
use crate::system::create_map::CreateMapData as Data;

pub struct CompassRose {}

impl<'a> CompassRose {
  /// Create the "compass rose" demo.
  pub fn build(&mut self, data: &mut Data<'a>) {
    let spawn_room = create_room!(data, "Spawn Room", "Dark olive trees crowd in on all sides, the air steams with the mist of a warm recent rain, midges hang in the air.");
    if let Some(player_id) = data.player_resource.0 {
      is_in_room!(data, get_entity!(data, player_id), RoomId(spawn_room.id()));
      write_effect_event!(
        data,
        Effect::EntityLooksAround(EntityLooksAround {
          entity_id: player_id.into(),
        })
      );
    }
    let _mushroom = create_object!(
      data,
      "Mushroom",
      "A speckled mushroom grows out of the sodden earth, on a long stalk.",
      RoomId(spawn_room.id())
    );
    let _goblin = create_actor!(
      data,
      "Goblin",
      "The goblin is short, stout, and ugly.",
      Gender::Male,
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
