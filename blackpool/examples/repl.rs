#![allow(unused_imports)]
#![allow(unused_macros)]

use std::io::{stdin, stdout, Write};

use blackpool::scripting_language::virtual_machine::VirtualMachine;
use blackpool::*;

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  let mut vm = VirtualMachine::new();
  loop {
    print!("> ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).expect("failed to read input");
    if line.is_empty() {
      break;
    }
    vm.interpret(&line).ok();
  }
  trace_exit!();
}
