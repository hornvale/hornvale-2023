use specs::prelude::*;

/// The `IsASpawnRoom` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[storage(NullStorage)]
pub struct IsASpawnRoom;
