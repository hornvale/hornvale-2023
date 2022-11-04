use crate::components::is_in_room::IsInRoom;

pub mod id;
use id::Id;

/// The `Entity` type.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Entity {
  /// The unique ID.
  pub id: Id,
  /// Which room it is in.
  pub is_in_room: Option<IsInRoom>,
}
