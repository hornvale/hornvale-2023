/// The `Severity` enum.
///
/// This is used to determine the relative priorities of competing stimuli by
/// assigning each a label indicating how much danger it poses to the being.
///
/// Below, a combat scenario is used to illustrate the different levels, but
/// it should be noted that these levels are used for any kind of signal that
/// an actor might perceive.  Infiltration and combat are merely a good, clear
/// example.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Severity {
  /// This will be processed, but is impossible to act on.  This might be the
  /// result of scripting, a magical effect, etc.
  Ignore,
  /// The perceiver will only take notice if they're otherwise idling.  This is
  /// laying-on-the-floor-playing-with-the-carpet levels of idleness/boredom.
  Idle,
  /// The perceiver, if idle, will find this interesting and contemplate it
  /// casually.
  Info,
  /// The perceiver will notice something and may or may not investigate.  They
  /// will prick their ears, though...
  Notice,
  /// The perceiver will investigate.  Perhaps not quite ready to sound the
  /// alarm, but will get there.
  Warning,
  /// The perceiver will sound an alarm and engage.
  Danger,
  /// The perceiver is engaged in active combat.  Every faculty is focused on
  /// the fight.
  Critical,
  /// The perceiver is losing the fight.  A creature without strong morale or
  /// heavy social pressure will attempt to flee to preserve its life.
  Alert,
  /// The perceiver is losing and death is imminent.  The situation and the
  /// creature iself are desperate.  Morale is likely to fail.
  Emergency,
  /// Used for controlling entities programmatically.
  Compulsory,
}
