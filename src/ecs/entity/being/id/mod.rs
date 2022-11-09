use super::super::PlayerId;
use specs::prelude::*;
use specs::world::Index;

/// The `BeingId` type.
///
/// We do this so that we can perform some compile-time type-checking with IDs.
#[derive(
  Clone, Component, Copy, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize,
)]
#[repr(transparent)]
pub struct Id(pub Index);

impl From<PlayerId> for Id {
  fn from(id: PlayerId) -> Self {
    Self(id.0)
  }
}
