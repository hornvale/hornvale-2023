use super::world::World;

/// A simple test perceptual dissonance.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PerceptualDissonance {
  pub world: World,
}

impl PerceptualDissonance {
  pub fn new(world: World) -> Self {
    Self { world }
  }

  pub fn get_goal_set(&self, _real_world: &World) -> World {
    self.world.clone()
  }
}
