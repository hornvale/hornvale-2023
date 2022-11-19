use crate::initiative::Initiative;
use specs::prelude::*;

/// The `HasInitiative` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasInitiative(pub Initiative);
