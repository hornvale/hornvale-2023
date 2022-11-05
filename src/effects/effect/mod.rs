use crate::entity::id::Id as EntityId;
use crate::room::id::Id as RoomId;

/// The `Effect` enum.
#[derive(Clone, Debug)]
pub enum Effect {
  /// An entity walks from Room 1 -> Room 2.
  EntityWalksBetweenRooms(EntityId, RoomId, RoomId),
}
