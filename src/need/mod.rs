/// The `Need` enum.
///
/// This is basically a de-generalization of Maslow's Hierarchy of Needs.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Need {
  Beauty,
  Novelty,
  Meaning,
  Family,
  Friendship,
  Intimacy,
  Trust,
  Acceptance,
  Health,
  Prosperity,
  Air,
  Heat,
  Clothing,
  Hygiene,
  Light,
  Water,
  Urination,
  Food,
  Excretion,
  Shelter,
  Sleep,
}
