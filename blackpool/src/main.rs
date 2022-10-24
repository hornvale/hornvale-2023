use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Error as IoError, Write};
use std::process::exit;

#[macro_use]
extern crate thiserror;

use blackpool::scripting_language::virtual_machine::error::Error as VirtualMachineError;
use blackpool::scripting_language::virtual_machine::VirtualMachine;
use blackpool::*;

/// Errors encountered in compiling or executing a script.
#[derive(Debug, Error)]
pub enum Error {
  /// An input/output error occurred.
  #[error("an input/output error occurred ({0})")]
  InputOutputError(#[from] IoError),
  /// A virtual machine error occurred.
  #[error("a virtual machine error occurred ({0})")]
  VirtualMachineError(#[from] VirtualMachineError),
}

#[named]
fn repl(vm: &mut VirtualMachine) -> Result<(), Error> {
  trace_enter!();
  loop {
    print!("> ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).expect("failed to read input");
    if line.is_empty() {
      break;
    }
    trace_var!(line);
    match vm.interpret(&line) {
      Ok(()) => println!("OK"),
      Err(error) => println!("Error: {}", error),
    }
  }
  trace_exit!();
  Ok(())
}

#[named]
fn run_file(vm: &mut VirtualMachine, path: &str) -> Result<(), Error> {
  trace_enter!();
  trace_var!(path);
  let source = read_to_string(path)?;
  vm.interpret(&source)?;
  Ok(())
}

fn main() -> Result<(), Error> {
  use pretty_env_logger::env_logger::builder;
  let _ = builder().is_test(true).try_init();
  let args: Vec<String> = args().collect();
  let mut vm = VirtualMachine::new();
  match args.len() {
    1 => repl(&mut vm),
    2 => run_file(&mut vm, &args[1]),
    _ => exit(-1),
  }
}
