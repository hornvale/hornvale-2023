use specs::prelude::*;
use specs::world::Index;

use super::object::id::Id as ObjectId;
use super::player::id::Id as PlayerId;
use super::room::id::Id as RoomId;

/// The `EntityId` type.
///
/// We do this so that we can perform some compile-time type-checking with IDs.
#[derive(
  Clone, Component, Copy, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize,
)]
pub struct Id(pub Index);

impl From<ObjectId> for Id {
  fn from(id: ObjectId) -> Self {
    Self(id.0)
  }
}

impl From<PlayerId> for Id {
  fn from(id: PlayerId) -> Self {
    Self(id.0)
  }
}

impl From<RoomId> for Id {
  fn from(id: RoomId) -> Self {
    Self(id.0)
  }
}
