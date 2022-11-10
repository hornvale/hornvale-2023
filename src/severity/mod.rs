/// The `Severity` enum.
///
/// This is used to determine the relative priorities of competing stimuli by
/// assigning each a label indicating how much danger it poses to the being.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Severity {
  Trace,
  Info,
  Notice,
  Warning,
  Danger,
  Critical,
  Alert,
  Emergency,
}
