use crate::ecs::entity::RoomId;

/// The `Destination` enum.
///
/// At some point, I should add support for doors.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Variation)]
pub enum Destination {
  /// A message, in lieu of actual travel.
  Message(String),
  /// Another room, referenced by its ID.
  Room(RoomId),
}
