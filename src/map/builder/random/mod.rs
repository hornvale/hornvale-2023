use super::{CompassRoseBuilder, TrivialMazeBuilder};
use crate::system::create_map::CreateMapData as Data;
use rand::prelude::*;

pub struct Random {}

impl<'a> Random {
  /// Create a "trivial maze" demo.
  pub fn build(&mut self, data: &mut Data<'a>) {
    let rng = &mut data.random_resource.0;
    if rng.gen::<bool>() {
      CompassRoseBuilder {}.build(data);
    } else {
      TrivialMazeBuilder {}.build(data);
    }
  }
}
