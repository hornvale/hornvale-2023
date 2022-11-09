use specs::prelude::*;

/// The `HasName` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasName(pub String);
