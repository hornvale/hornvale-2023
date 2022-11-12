use crate::ailment::Ailment;

/// The `Condition` enum.
///
/// This is intended to represent the various conditions that can be in play.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Condition {
  HasAilment(Ailment),
  DoesNotHaveAilment(Ailment),
  InMidair,
  Underwater,
}
