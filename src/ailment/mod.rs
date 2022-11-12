/// The `Ailment` enum.
///
/// This represents the various ailments that an actor might have.
///
/// Some may or may not make sense depending on species, etc.  It's not likely
/// that a stone golem will contract a disease.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Ailment {
  Anosmic,
  Asleep,
  Blind,
  Confused,
  Deaf,
  Decapitated,
  Delusional,
  Exhausted,
  Febrile,
  Handless,
  HandsBroken,
  HandsDisabled,
  HandsSevered,
  Hemorrhagic,
  Hungry,
  Insensate,
  Legless,
  LegsBroken,
  LegsDisabled,
  LegsSevered,
  Mute,
  Nauseous,
  Paralyzed,
  Ravenous,
  Starving,
  Stunned,
  Unconscious,
}
