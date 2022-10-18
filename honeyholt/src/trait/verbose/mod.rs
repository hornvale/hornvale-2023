use std::fmt;

/// A full sentence, even a paragraph.
pub trait Verbose {
  fn honeyholt_display_verbose<'a>(&'a self) -> Box<dyn fmt::Display + 'a>;
}
