use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod map;
pub use map::Map as TileMap;

/// The `Tile` type.
///
/// In certain scenarios, we'll use a tile to refer to a room.  This is not to
/// say this is a roguelike; each square maps to a _room_ and not a walkable
/// position.  We don't need to worry about furnishings, foes, etc.  Just a
/// simple room map.  Don't make this weird.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Tile {
  Wall,
  Floor,
  Player,
}

impl Tile {
  /// Get string.
  pub fn get_str(&self) -> &'static str {
    use Tile::*;
    match &self {
      Wall => "<fg_ext238>#<reset>",
      Floor => "<fg_ext238>.<reset>",
      Player => "<fg_ext21>@<reset>",
    }
  }
}

impl Display for Tile {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    write!(formatter, "{}", self.get_str())
  }
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }

  #[test]
  pub fn test_display_tile_types() {
    init();
    assert_eq!(format!("{}", Tile::Wall), "#");
    assert_eq!(format!("{}", Tile::Floor), ".");
    assert_eq!(format!("{}", Tile::Player), "@");
  }
}
