use crate::io::error::Error;
use crate::io::interpreter::Interpreter;

pub struct Echo {}

impl Interpreter for Echo {
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
    let result = input.to_owned();
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
