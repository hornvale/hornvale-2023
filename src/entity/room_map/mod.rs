use std::collections::HashMap;

use super::EntityId;
use crate::room::RoomId;

/// The `EntityCollection` type.
#[derive(Clone, Debug, Default)]
pub struct RoomMap(pub HashMap<EntityId, RoomId>);

impl RoomMap {}
