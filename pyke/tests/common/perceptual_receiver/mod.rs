use super::world::World;

/// A simple test perceptual receiver.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PerceptualReceiver {
  world: World,
}

impl PerceptualReceiver {
  pub fn new(world: World) -> Self {
    Self { world }
  }

  pub fn receive_new(&mut self, new_world: &World) {
    self.world = new_world.clone();
  }
}
