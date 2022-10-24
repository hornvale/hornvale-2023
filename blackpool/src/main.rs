use std::env::args;
use std::fs::read_to_string;
use std::io::{self, BufRead, Error as IoError, Write};
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
pub fn repl<R, W>(vm: &mut VirtualMachine, mut input: R, mut output: W) -> Result<(), Error>
where
  R: BufRead,
  W: Write,
{
  trace_enter!();
  loop {
    write!(&mut output, "> ")?;
    output.flush()?;
    let mut line = String::new();
    input.read_line(&mut line)?;
    if line.is_empty() {
      break;
    }
    trace_var!(line);
    match vm.interpret(&line) {
      Ok(_) => {
        let value_option = vm.pop().ok();
        match value_option {
          Some(value) => writeln!(&mut output, "OK: {}", value)?,
          None => writeln!(&mut output, "OK")?,
        }
      },
      Err(error) => writeln!(&mut output, "Error: {}", error)?,
    }
  }
  trace_exit!();
  Ok(())
}

#[named]
pub fn run_file(vm: &mut VirtualMachine, path: &str) -> Result<(), Error> {
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
    1 => {
      let stdio = io::stdin();
      let input = stdio.lock();
      let output = io::stdout();
      repl(&mut vm, input, output)
    },
    2 => run_file(&mut vm, &args[1]),
    _ => exit(-1),
  }
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  #[named]
  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }

  #[named]
  #[test]
  #[should_panic]
  pub fn test() {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::default();
    run_file(&mut vm, "nonexistent file.txt").unwrap();
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test2() {
    init();
    trace_enter!();
    let mut vm = VirtualMachine::default();
    let mut output = Vec::new();
    let input = b"3 + 4";
    repl(&mut vm, &input[..], output).unwrap();
    trace_exit!();
  }
}
