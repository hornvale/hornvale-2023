use specs::prelude::*;

/// The `IsAnActor` component.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[storage(NullStorage)]
pub struct IsAnActor;
