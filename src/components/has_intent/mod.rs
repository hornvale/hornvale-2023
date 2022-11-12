use crate::intent::Intent;
use specs::prelude::*;

/// The `HasIntent` component.
#[derive(Clone, Component, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct HasIntent(pub Intent);
