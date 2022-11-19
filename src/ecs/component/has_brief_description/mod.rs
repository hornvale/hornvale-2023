use specs::prelude::*;

/// The `HasBriefDescription` type.
///
/// This provides a brief description of the entity.  We may offer a fleshier
/// description at some point, but all entities should have this.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasBriefDescription(pub String);
