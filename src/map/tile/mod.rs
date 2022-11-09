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
  Void,
  Wall,
  Floor,
  Player,
}

impl Tile {
  /// Get char.
  pub fn get_char(&self) -> char {
    use Tile::*;
    match &self {
      Void => ' ',
      Wall => '#',
      Floor => '.',
      Player => '@',
    }
  }

  /// Get color code.
  pub fn get_color_code(&self) -> u8 {
    use Tile::*;
    match &self {
      Void => 232,
      Wall => 238,
      Floor => 238,
      Player => 21,
    }
  }

  /// Get string.
  pub fn get_string(&self) -> String {
    format!("<fg_ext{}>{}<reset>", self.get_color_code(), self.get_char())
  }
}

impl Display for Tile {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    write!(formatter, "{}", self.get_string())
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
  pub fn test_display_tile_chars() {
    init();
    assert_eq!(Tile::Wall.get_char(), '#');
    assert_eq!(Tile::Floor.get_char(), '.');
    assert_eq!(Tile::Player.get_char(), '@');
  }
}
