use super::super::entity::RoomId;

/// The `SpawnRoom` resource.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SpawnRoom(pub Option<RoomId>);
