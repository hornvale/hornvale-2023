use specs::prelude::*;

/// The `IsABeing` component.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[storage(NullStorage)]
pub struct IsABeing;
