use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

/// The `Reverse` interpreter.
///
/// This just returns text reversed.
#[derive(Clone, Debug)]
pub struct Reverse {}

impl Interpreter for Reverse {
  fn get_initial_text(&self) -> Result<Option<String>, Error> {
    let result = None;

    Ok(result)
  }

  fn interpret(&mut self, input: &str) -> Result<Option<String>, Error> {
    let result = Some(input.chars().rev().collect::<String>());

    Ok(result)
  }
}
