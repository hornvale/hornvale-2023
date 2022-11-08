use specs::prelude::*;

/// The `HasName` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HasName(pub String);
