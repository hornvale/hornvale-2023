use crate::room::id::Id as RoomId;

/// The `Destination` enum.
///
/// At some point, I should add support for doors.
#[derive(Clone, Debug)]
pub enum Destination {
  /// A message, in lieu of actual travel.
  Message(String),
  /// Another room, referenced by its ID.
  Room(RoomId),
}
