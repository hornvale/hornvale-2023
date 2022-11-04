use crate::direction::Direction;

pub mod destination;
use destination::Destination;

/// The `Passage` enum.
#[derive(Clone, Debug)]
pub struct Passage {
  /// The direction in which this passage leads.
  pub direction: Direction,
  /// The destination to which this passage leads.
  pub destination: Destination,
}
