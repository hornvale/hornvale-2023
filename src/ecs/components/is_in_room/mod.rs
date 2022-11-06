use specs::prelude::*;

use super::super::entity::RoomId;

/// The `IsInRoom` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub struct IsInRoom(pub RoomId);
