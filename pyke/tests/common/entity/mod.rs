use super::perceptual_dissonance::PerceptualDissonance;
use super::perceptual_filter::PerceptualFilter;
use super::perceptual_receiver::PerceptualReceiver;
use super::world::World;

/// A simple test entity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
  pub name: String,
  pub perceptual_filter: PerceptualFilter,
  pub perceptual_receiver: PerceptualReceiver,
  pub perceptual_dissonance: PerceptualDissonance,
}

impl Entity {
  pub fn new(name: String, perceptual_filter: PerceptualFilter, desired_world: World) -> Self {
    let perceptual_receiver = PerceptualReceiver::new(perceptual_filter.world.clone());
    let perceptual_dissonance = PerceptualDissonance::new(desired_world.clone());
    Self {
      name,
      perceptual_filter,
      perceptual_receiver,
      perceptual_dissonance,
    }
  }

  pub fn process_dissonance(&mut self) {
    self.perceptual_receiver.receive_new(&self.perceptual_filter.world);
    let _dissonance = self.perceptual_dissonance.get_goal_set(&self.perceptual_receiver.world);
  }
}
