use crate::ai::Ai;
use specs::prelude::*;

/// The `HasAi` component.
///
/// This provides an AI engine that can be consulted for the actor's next move.
#[derive(Clone, Component, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasAi(pub Ai);
