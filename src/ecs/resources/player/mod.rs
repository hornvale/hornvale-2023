use super::super::entity::PlayerId;

/// The `Player` resource.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Player(pub Option<PlayerId>);
