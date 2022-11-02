///! A simple REPL that echoes your input.
use std::io::{self};

use hornvale::io::interpreter::echo::Echo;
use hornvale::io::repl::Repl;

fn main() {
  let stdio = io::stdin();
  let input = stdio.lock();
  let output = io::stdout();
  let interpreter = Echo {};
  let mut repl = Repl::new(input, output, interpreter);
  repl.run().unwrap();
}
