use crate::room::id::Id as RoomId;

/// The `IsInRoom` type.
#[derive(Clone, Debug, Default, Display, Eq, Hash, PartialEq)]
pub struct IsInRoom(pub RoomId);
