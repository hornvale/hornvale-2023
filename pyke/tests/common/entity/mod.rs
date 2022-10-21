use super::perceptual_filter::PerceptualFilter;
use super::perceptual_receiver::PerceptualReceiver;

/// A simple test entity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
  pub name: String,
  pub perceptual_filter: PerceptualFilter,
  pub perceptual_receiver: PerceptualReceiver,
}

impl Entity {
  pub fn new(name: String, perceptual_filter: PerceptualFilter) -> Self {
    let perceptual_receiver = PerceptualReceiver::new(perceptual_filter.world.clone());
    Self {
      name,
      perceptual_filter,
      perceptual_receiver,
    }
  }
}
