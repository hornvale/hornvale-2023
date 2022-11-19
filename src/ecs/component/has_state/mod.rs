use crate::goap::state::State;
use specs::prelude::*;

/// The `HasState` component.
///
/// This provides storage for tracking some of an actor's internal state.
///
/// Realistically, this will be calculated from other components attached to
/// the entity.
#[derive(Clone, Component, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasState(pub State);
