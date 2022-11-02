use std::io::{self, BufRead, StdinLock, Stdout, Write};

use crate::io::error::Error;
use crate::io::interpreter::parser::Parser as ParserInterpreter;
use crate::io::interpreter::Interpreter;
use crate::parsing::two_word::TwoWord;

/// The `Repl` type.
///
/// REPL (Read-Eval-Print-Loop) is typically used to refer to interactive
/// programming language interfaces.  It reads user input, evaluates it, prints
/// the response, and then loops.
///
/// This is a simple, interactive interface to use with Hornvale.  The problem
/// is that it is synchronous, so we can't use this for the full project... but
/// for simple demonstrations and early development, it should be sufficient.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Repl<R: BufRead, W: Write, I: Interpreter> {
  #[derivative(Debug = "ignore")]
  pub input: R,
  #[derivative(Debug = "ignore")]
  pub output: W,
  #[derivative(Debug = "ignore")]
  pub interpreter: I,
}

impl<R: BufRead, W: Write, I: Interpreter> Repl<R, W, I> {
  /// Constructor.
  #[named]
  pub fn new(input: R, output: W, interpreter: I) -> Self {
    Self {
      input,
      output,
      interpreter,
    }
  }

  /// Runloop.
  #[named]
  pub fn run(&mut self) -> Result<(), Error> {
    let initial_text = self.interpreter.get_initial_text()?;
    writeln!(&mut self.output, "{}", initial_text.unwrap_or_default())?;
    loop {
      write!(&mut self.output, "> ")?;
      self.output.flush()?;
      let mut line = String::new();
      self.input.read_line(&mut line)?;
      if &line == "quit" {
        break;
      }

      // Note that the string comes in with a trailing newline.
      let response = self.interpreter.interpret(line.trim())?;

      writeln!(&mut self.output, "{}", response.unwrap_or_default())?;
    }

    Ok(())
  }
}

impl Default for Repl<StdinLock<'_>, Stdout, ParserInterpreter<TwoWord>> {
  #[named]
  fn default() -> Self {
    let stdio = io::stdin();
    let input = stdio.lock();
    let output = io::stdout();
    let interpreter = ParserInterpreter::new(TwoWord {});

    Self::new(input, output, interpreter)
  }
}
