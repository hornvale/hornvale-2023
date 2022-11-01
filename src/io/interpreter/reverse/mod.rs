use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

/// The `Reverse` interpreter.
///
/// This just returns text reversed.
#[derive(Clone, Debug)]
pub struct Reverse {}

impl Interpreter for Reverse {
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
    let result = Some(input.chars().rev().collect::<String>());
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
