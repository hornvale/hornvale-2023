use std::io::{self, BufRead, Error as IoError, StdinLock, Stdout, Write};

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
pub struct Repl<R: BufRead, W: Write> {
  #[derivative(Debug = "ignore")]
  pub input: R,
  #[derivative(Debug = "ignore")]
  pub output: W,
}

impl<R: BufRead, W: Write> Repl<R, W> {
  /// Constructor.
  #[named]
  pub fn new(input: R, output: W) -> Self {
    trace_enter!();
    let result = Self { input, output };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Runloop.
  #[named]
  pub fn run(&mut self) -> Result<(), IoError> {
    trace_enter!();
    loop {
      write!(&mut self.output, "> ")?;
      self.output.flush()?;
      let mut line = String::new();
      self.input.read_line(&mut line)?;
      if &line == "quit" {
        break;
      }
      trace_var!(line);
      writeln!(&mut self.output, "{}", line)?;
    }
    trace_exit!();
    Ok(())
  }
}

impl Default for Repl<StdinLock<'_>, Stdout> {
  #[named]
  fn default() -> Self {
    trace_enter!();
    let stdio = io::stdin();
    let input = stdio.lock();
    let output = io::stdout();
    let result = Self::new(input, output);
    trace_var!(result);
    trace_exit!();
    result
  }
}
