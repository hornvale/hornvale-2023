use crate::ecs::entity::EntityId;
use crate::ecs::entity::RoomId;

/// The `Effect` enum.
#[derive(Clone, Debug)]
pub enum Effect {
  /// An entity walks from Room 1 -> Room 2.
  EntityWalksBetweenRooms(EntityId, RoomId, RoomId),
}
