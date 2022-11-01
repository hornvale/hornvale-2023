use crate::io::error::Error;
use crate::io::interpreter::reverse::Reverse;
use crate::io::interpreter::Interpreter;
use std::io::{self, BufRead, StdinLock, Stdout, Write};

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
    trace_enter!();
    let result = Self {
      input,
      output,
      interpreter,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Runloop.
  #[named]
  pub fn run(&mut self) -> Result<(), Error> {
    trace_enter!();
    write!(&mut self.output, "{}", self.interpreter.get_initial_text())?;
    loop {
      write!(&mut self.output, "> ")?;
      self.output.flush()?;
      let mut line = String::new();
      self.input.read_line(&mut line)?;
      if &line == "quit" {
        break;
      }
      trace_var!(line);
      // Note that the string comes in with a trailing newline.
      let response = self.interpreter.interpret(line.trim())?;
      trace_var!(response);
      writeln!(&mut self.output, "{}", response)?;
    }
    trace_exit!();
    Ok(())
  }
}

impl Default for Repl<StdinLock<'_>, Stdout, Reverse> {
  #[named]
  fn default() -> Self {
    trace_enter!();
    let stdio = io::stdin();
    let input = stdio.lock();
    let output = io::stdout();
    let interpreter = Reverse {};
    let result = Self::new(input, output, interpreter);
    trace_var!(result);
    trace_exit!();
    result
  }
}
