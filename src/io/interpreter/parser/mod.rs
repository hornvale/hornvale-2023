use std::fmt::Debug;

use crate::io::error::Error;
use crate::io::interpreter::Interpreter;
use crate::parsing::parser::Parser as ParserTrait;

/// The `Parser` interpreter, which passes input to a parser.
#[derive(Clone, Debug)]
pub struct Parser<P: ParserTrait + Debug> {
  pub parser: Box<P>,
}

impl<P> Parser<P>
where
  P: ParserTrait + Debug,
{
  /// Constructor.
  #[named]
  pub fn new(parser: P) -> Self {
    trace_enter!();
    trace_var!(parser);
    let parser = Box::new(parser);
    trace_var!(parser);
    let result = Self { parser };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl<P> Interpreter for Parser<P>
where
  P: ParserTrait + Debug,
{
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
    let result = self.parser.parse_input(input)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
