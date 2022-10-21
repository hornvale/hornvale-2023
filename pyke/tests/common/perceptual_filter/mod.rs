use super::world::World;

/// A simple test perceptual filter
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PerceptualFilter {
  pub world: World,
}

impl PerceptualFilter {
  pub fn new(world: World) -> Self {
    Self { world }
  }

  pub fn get_last(&self) -> World {
    self.world.clone()
  }
}
