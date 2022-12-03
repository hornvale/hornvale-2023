use crate::intent::Intent;
use specs::prelude::*;

/// The `HasIntent` component.
#[derive(Clone, Component, Debug)]
pub struct HasIntent(pub Intent);
