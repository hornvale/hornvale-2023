use super::perceptual_filter::PerceptualFilter;

/// A simple test entity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
  pub name: String,
  pub perceptual_filter: PerceptualFilter,
}

impl Entity {
  pub fn new(name: String, perceptual_filter: PerceptualFilter) -> Self {
    Self {
      name,
      perceptual_filter,
    }
  }
}
