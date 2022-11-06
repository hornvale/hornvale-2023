use crate::components::is_in_room::IsInRoom;

pub mod collection;
pub use collection::Collection as EntityCollection;
pub mod id;
use id::Id;
pub use id::Id as EntityId;
pub mod room_map;
pub use room_map::RoomMap as EntityRoomMap;

/// The `Entity` type.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Entity {
  /// The unique ID.
  pub id: Id,
  /// Which room it is in.
  pub is_in_room: Option<IsInRoom>,
}

impl Entity {
  pub fn get_id(&self) -> &EntityId {
    &self.id
  }
}
