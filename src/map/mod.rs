use std::collections::HashMap;

use crate::room::id::Id as RoomId;
use crate::room::Room;

pub mod builder;

/// The `Map` type.
///
///
#[derive(Clone, Debug, Default)]
pub struct Map {
  /// A simple hash map of the rooms.
  pub rooms: HashMap<RoomId, Room>,
  /// The spawn room ID.
  pub spawn_room_id: Option<RoomId>,
}

impl Map {
  /// Constructor.
  pub fn new() -> Self {
    let rooms = HashMap::new();
    let spawn_room_id = None;
    Self { rooms, spawn_room_id }
  }
}
