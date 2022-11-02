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

  pub fn new(parser: P) -> Self {
    let parser = Box::new(parser);

    Self { parser }
  }
}

impl<P> Interpreter for Parser<P>
where
  P: ParserTrait + Debug,
{
  fn get_initial_text(&self) -> Result<Option<String>, Error> {
    let result = None;

    Ok(result)
  }

  fn interpret(&mut self, input: &str) -> Result<Option<String>, Error> {
    let result = self.parser.parse_input(input)?;

    Ok(result)
  }
}
