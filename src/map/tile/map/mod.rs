use super::Tile;
use crate::entity::RoomId;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// The `TileMap` data structure.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Map {
  /// The actual map.
  pub map: Vec<Tile>,
  /// The visibility mask.
  pub visibility: Vec<bool>,
  /// Cooresponding rooms.
  pub rooms: Vec<Option<RoomId>>,
  /// Room to Index map.
  pub room_index_map: HashMap<RoomId, usize>,
  /// Room to Coordinates map.
  pub room_coords_map: HashMap<RoomId, (usize, usize)>,
  /// The height of the map.
  pub height: usize,
  /// The width of the map.
  pub width: usize,
  /// The coordinates of the player.
  pub player_coordinates: Option<(usize, usize)>,
}

impl Map {
  /// Constructor.
  pub fn new(o_width: usize, o_height: usize) -> Self {
    let width = 2 * o_width + 1;
    let height = 2 * o_height + 1;
    let length = height * width;
    let map = vec![Tile::Wall; length];
    let visibility = vec![false; length];
    let rooms = vec![None; length];
    let room_index_map = HashMap::new();
    let room_coords_map = HashMap::new();
    let player_coordinates = None;
    Self {
      map,
      visibility,
      rooms,
      room_index_map,
      room_coords_map,
      height,
      width,
      player_coordinates,
    }
  }

  /// Transformed x.
  pub fn get_x(&self, x: usize) -> usize {
    2 * x + 1
  }

  /// Transformed y.
  pub fn get_y(&self, y: usize) -> usize {
    2 * y + 1
  }

  /// Transformed index.
  pub fn get_index(&self, x: usize, y: usize) -> usize {
    self.get_y(y) * self.width + self.get_x(x)
  }

  /// Mark coordinates and area surrounding them visible.
  pub fn mark_visible(&mut self, x: usize, y: usize) {
    for i_y in y.saturating_sub(2)..=y + 2 {
      for i_x in x.saturating_sub(2)..=x + 2 {
        if i_y >= self.height || i_x >= self.width {
          continue;
        }
        self.visibility[i_y as usize * self.width + i_x as usize] = true;
      }
    }
  }

  /// Set a room ID.
  pub fn set_room_id(&mut self, room_id: RoomId, x: usize, y: usize) {
    let index = self.get_index(x, y);
    self.rooms[index] = Some(room_id);
    self.room_index_map.insert(room_id, index);
    self.room_coords_map.insert(room_id, (self.get_x(x), self.get_y(y)));
  }
}

impl Display for Map {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    let mut map = self.map.clone();
    if let Some((p_x, p_y)) = self.player_coordinates {
      map[p_y * self.width + p_x] = Tile::Player;
    }
    let mut strings = Vec::new();
    for y in 0..self.height {
      for x in 0..self.width {
        let index = y * self.width + x;
        if self.visibility[index] {
          strings.push(map[index].get_string());
        } else {
          strings.push(Tile::Void.get_string());
        }
      }
      strings.push("\n".to_string());
    }
    write!(formatter, "{}", strings.join(""))
  }
}

#[cfg(test)]
pub mod test {

  use super::super::Tile;
  use super::*;
  use pretty_env_logger::env_logger::builder;
  use rand::prelude::*;
  use std::env::set_var;

  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }

  #[test]
  pub fn test_display_tile_map() {
    init();
    let mut rng = rand::thread_rng();
    let height = 25;
    let width = 80;
    let mut map = Map::new(height, width);
    use Tile::*;
    for y in 1..height - 1 {
      let is_on_bottom = y == height - 2;
      for x in 1..width - 1 {
        let is_on_right = x == width - 1;
        if y % 2 != 0 && x % 2 != 0 {
          map.map[(y * width) + x] = Floor;
          let right: bool = rng.gen();
          if is_on_bottom || !is_on_right && right {
            map.map[(y * width) + x + 1] = Floor;
          } else {
            map.map[((y + 1) * width) + x] = Floor;
          }
        }
      }
    }
    println!("{}", map);
  }
}
