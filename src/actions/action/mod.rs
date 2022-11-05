use crate::actions::error::Error;
use crate::components::is_in_room::IsInRoom;
use crate::direction::Direction;
use crate::entity::id::Id as EntityId;
use crate::passage::destination::Destination;
use crate::passage::Passage;
use crate::world::World;

/// The `Action` enum.
pub enum Action {
  /// Look around at current surroundings.
  LookAround(EntityId),
  /// An entity looks in a specific direction.
  LookDirection(EntityId, Direction),
  /// Go in a specific direction.
  GoDirection(EntityId, Direction),
}

impl Action {
  /// Execute.
  pub fn execute(&mut self, world: &mut World) -> Result<Option<String>, Error> {
    use Action::*;
    let result = match &self {
      LookAround(entity_id) => {
        let entity = world.entities.get(entity_id).unwrap();
        let room_id = &entity.is_in_room.as_ref().unwrap().0;
        let map = world.map.as_ref().unwrap();
        let room = map.rooms.get(room_id).unwrap();
        let room_description = &room.has_description.as_ref().unwrap().brief;
        Some(room_description.to_string())
      },
      GoDirection(entity_id, direction) => {
        let mut entity = world.entities.get_mut(entity_id).unwrap();
        let room_id = &entity.is_in_room.as_ref().unwrap().0;
        let map = world.map.as_ref().unwrap();
        let room = map.rooms.get(room_id).unwrap();
        let passages = room.has_passages.as_ref().unwrap();
        match passages.get_passage_to(direction) {
          Some(Passage {
            destination: Destination::Room(room_id),
            ..
          }) => {
            entity.is_in_room = Some(IsInRoom(room_id.clone()));
            Some(format!("You walk to the {}.", direction.get_lowercase()))
          },
          _ => Some("You bump your noggin into the wall.".to_string()),
        }
      },
      LookDirection(entity_id, direction) => {
        let entity = world.entities.get_mut(entity_id).unwrap();
        let room_id = &entity.is_in_room.as_ref().unwrap().0;
        let map = world.map.as_ref().unwrap();
        let room = map.rooms.get(room_id).unwrap();
        let passages = room.has_passages.as_ref().unwrap();
        match passages.get_passage_to(direction) {
          Some(Passage {
            destination: Destination::Room(room_id),
            ..
          }) => {
            let room = map.rooms.get(room_id).unwrap();
            let room_description = &room.has_description.as_ref().unwrap().brief;
            Some(format!(
              "You look to the {}.\n\n{}",
              direction.get_lowercase(),
              room_description
            ))
          },
          _ => Some(format!(
            "You don't see any passages leading {}.",
            direction.get_lowercase()
          )),
        }
      },
    };
    Ok(result)
  }
}
