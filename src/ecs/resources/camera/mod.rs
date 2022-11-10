use super::super::entity::EntityId;

/// The `Camera` resource.
///
/// This controls which entity drives output and the first- and second-person
/// behaviors.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[repr(transparent)]
pub struct Camera(pub Option<EntityId>);
