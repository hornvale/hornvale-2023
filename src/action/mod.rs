use crate::ecs::entity::EntityId;
use crate::ecs::entity::ObjectId;
use crate::map::Direction;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Action {
  Look { entity_id: EntityId },
  LookDirection { entity_id: EntityId, direction: Direction },
  LookAtObject { entity_id: EntityId, object_id: ObjectId },
  MoveDirection { entity_id: EntityId, direction: Direction },
}
