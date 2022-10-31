use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

pub struct Reverse {}

impl Interpreter for Reverse {
  #[named]
  fn get_initial_text(&self) -> &str {
    trace_enter!();
    let result = "";
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn interpret(&mut self, input: &str) -> Result<String, Error> {
    trace_enter!();
    trace_var!(input);
    let result = input.chars().rev().collect::<String>();
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
