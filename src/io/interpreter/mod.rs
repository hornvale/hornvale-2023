use crate::io::error::Error;

pub mod echo;
pub mod reverse;

/// The `Interpreter` trait.
pub trait Interpreter {
  /// Returns some initial text prior to interpreting any input.
  fn get_initial_text(&self) -> &str;
  /// Handles incoming input and returns a response.
  fn interpret(&mut self, input: &str) -> Result<String, Error>;
}
