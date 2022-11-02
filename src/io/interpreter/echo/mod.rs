use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

/// The `Echo` interpreter.
///
/// This just echoes input right back to the screen.
#[derive(Clone, Debug)]
pub struct Echo {}

impl Interpreter for Echo {
  fn get_initial_text(&self) -> Result<Option<String>, Error> {
    let result = None;

    Ok(result)
  }

  fn interpret(&mut self, input: &str) -> Result<Option<String>, Error> {
    let result = Some(input.to_owned());

    Ok(result)
  }
}
