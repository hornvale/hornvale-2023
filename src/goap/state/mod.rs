/// The `State` struct.
///
/// This represents some description of the world that an actor might use.
///
/// For instance, a very simple organism might track only their hunger in a
/// State object.  Then they might specify that they do not want to be hungry.
/// The resulting discrepancy can be used in planning to navigate the critter
/// to some food and eat it.
///
/// After sleeping on it, I've decided that I'm going to make this as simple
/// as possible: just a bitfield, basically.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State {
  /// The actual values managed in this struct.
  pub values: u64,
  /// A mask indicating which atoms do not matter.
  pub mask: u64,
}

impl State {
  /// Calculate meaningful distance.
  pub fn get_distance(&self, other: &State) -> usize {
    let difference = (self.values & self.mask) ^ (other.values & self.mask);
    difference.count_ones() as usize
  }
}
