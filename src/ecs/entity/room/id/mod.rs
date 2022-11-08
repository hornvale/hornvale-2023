use specs::prelude::*;
use specs::world::Index;

/// The `RoomId` type.
///
/// We do this so that we can perform some compile-time type-checking with IDs.
#[derive(
  Clone, Component, Copy, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize,
)]
pub struct Id(pub Index);
