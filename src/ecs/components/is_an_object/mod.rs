use specs::prelude::*;

/// The `IsAnObject` component.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[storage(NullStorage)]
pub struct IsAnObject;
