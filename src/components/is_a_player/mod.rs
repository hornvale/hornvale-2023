use specs::prelude::*;

/// The `IsAPlayer` component.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[storage(NullStorage)]
pub struct IsAPlayer;
