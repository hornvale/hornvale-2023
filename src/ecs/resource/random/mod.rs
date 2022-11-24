use rand_seeder::{SipHasher, SipRng};

/// The `Random` resource.
///
/// This makes a random number generator available to processes that need one.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Random(pub SipRng);

impl Default for Random {
  fn default() -> Self {
    Self(SipHasher::from("goat boy").into_rng())
  }
}
