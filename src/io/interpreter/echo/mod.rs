use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

/// The `Echo` interpreter.
///
/// This just echoes input right back to the screen.
#[derive(Clone, Debug)]
pub struct Echo {}

impl Interpreter for Echo {
  #[named]
  fn get_initial_text(&self) -> Result<Option<String>, Error> {
    trace_enter!();
    let result = None;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
  #[named]
  fn interpret(&mut self, input: &str) -> Result<Option<String>, Error> {
    trace_enter!();
    trace_var!(input);
    let result = Some(input.to_owned());
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
