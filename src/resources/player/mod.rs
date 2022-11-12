use crate::entity::PlayerId;

/// The `Player` resource.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[repr(transparent)]
pub struct Player(pub Option<PlayerId>);
