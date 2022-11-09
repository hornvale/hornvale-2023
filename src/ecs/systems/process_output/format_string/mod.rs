use super::*;
use crate::formatting::format_string as inner_format_string;

impl ProcessOutput {
  /// Format output string.
  pub fn format_string(&mut self, string: &str) -> String {
    inner_format_string(string)
  }
}
