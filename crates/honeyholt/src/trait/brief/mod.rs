use std::fmt;

/// A short phrase.
pub trait Brief {
  fn honeyholt_display_brief<'a>(&'a self) -> Box<dyn fmt::Display + 'a>;
}
