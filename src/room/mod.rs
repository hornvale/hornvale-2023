use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::components::has_description::HasDescription;
use crate::components::has_passages::HasPassages;

pub mod id;
use id::Id;

/// The `Room` type.
///
/// To be clear, we're using this in the sense that has developed over nearly
/// fifty years -- this might be indoors, outdoors, massive, tiny, a closet,
/// the southern end of an amphitheatre, a state of unbeing, whatever.
#[derive(Clone, Debug, Default)]
pub struct Room {
  /// Unique identifier referring only to this room.
  pub id: Id,
  /// Does this room have a description?
  pub has_description: Option<HasDescription>,
  /// Does this room have passages leading elsewhere?
  pub has_passages: Option<HasPassages>,
}

impl Display for Room {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", self)
  }
}
