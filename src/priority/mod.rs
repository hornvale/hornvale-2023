/// The `Priority` enum.
///
/// This is used to determine the relative priorities of competing actions.
///
/// When an entity is intending to perform an action, that intent is assigned
/// a priority according to the conditions that formed the intent.  Depending
/// on the updates that entity receives, it may choose to switch intents.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Priority {
  /// The action, if there is one, is a non-priority.  This is likely to be an
  /// artificial state resulting from magic or programmatic control.
  Negligible,
  /// The action is being pursued on a whim, with little or no investment.
  Lowest,
  /// Nothing better to do...
  VeryLow,
  /// Casually entered into, for pleasure or the future.
  Low,
  /// Not so important.
  ModeratelyLow,
  /// Meh.
  Moderate,
  /// Important.  
  ModeratelyHigh,
  /// Critically important, a matter of life-and-limb.
  High,
  /// Absolutely critical.  A matter of life-and-death.
  VeryHigh,
  /// No natural means will dissuade the creature.  This is being treated as a
  /// matter of life-and-death, even life-on-earth existential crisis.
  Highest,
  /// The creature will persist no matter what.  This is likely artificial, due
  /// to magic or programmatic control.
  Compulsory,
}
