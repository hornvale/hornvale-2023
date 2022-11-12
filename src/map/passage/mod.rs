use super::Direction;

use crate::entity::RoomId;

pub mod destination;
use destination::Destination;

/// The `Passage` enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Passage {
  /// The direction in which this passage leads.
  pub direction: Direction,
  /// The room ID from which this passage leads.
  pub from: RoomId,
  /// The destination to which this passage leads.
  pub to: Destination,
}
