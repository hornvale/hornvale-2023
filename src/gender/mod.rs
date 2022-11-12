/// The `Gender` enum.
///
/// This represents the genders in the game.  This may (likely will) grow with
/// time.  At this time, it's only meant as a grammatical construct, nothing
/// deeper.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Gender {
  // He/Him.
  Male,
  // She/Her.
  Female,
  // Does not have gender.  It/It.  Not to be confused with agender people.
  Genderless,
  // Does not identify as male or female.  They/Them.
  Nonbinary,
  // Is not yet identified as male or female.  They/Them.
  Unknown,
}
