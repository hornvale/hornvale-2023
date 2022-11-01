///! A simple REPL that echoes your input, but in reverse.
use std::io::{self, BufRead, StdinLock, Stdout, Write};

use hornvale::io::interpreter::reverse::Reverse;
use hornvale::io::repl::Repl;

fn main() {
  let stdio = io::stdin();
  let input = stdio.lock();
  let output = io::stdout();
  let interpreter = Reverse {};
  let mut repl = Repl::new(input, output, interpreter);
  repl.run().unwrap();
}
