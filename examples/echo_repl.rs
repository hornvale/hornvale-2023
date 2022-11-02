use hornvale::io::interpreter::echo::Echo;
use hornvale::io::repl::Repl;
use std::io::{self};

///! A simple REPL that echoes your input.
fn main() {
  let stdio = io::stdin();
  let input = stdio.lock();
  let output = io::stdout();
  let interpreter = Echo {};
  let mut repl = Repl::new(input, output, interpreter);
  repl.run().unwrap();
}
