use std::env::args;
use std::fs::read_to_string;
use std::io::{self, BufRead, Error as IoError, Write};
use std::process::exit;

#[macro_use]
extern crate thiserror;

use hornvale::scripting_language::virtual_machine::error::Error as VirtualMachineError;
use hornvale::scripting_language::virtual_machine::VirtualMachine;
use hornvale::*;

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

pub fn repl<R, W>(vm: &mut VirtualMachine, mut input: R, mut output: W) -> Result<(), Error>
where
  R: BufRead,
  W: Write,
{
  loop {
    write!(&mut output, "> ")?;
    output.flush()?;
    let mut line = String::new();
    input.read_line(&mut line)?;
    if line.is_empty() {
      break;
    }

    match vm.interpret(&line) {
      Ok(_) => writeln!(&mut output, "OK")?,
      Err(error) => writeln!(&mut output, "Error: {}", error)?,
    }
  }

  Ok(())
}

pub fn run_file(vm: &mut VirtualMachine, path: &str) -> Result<(), Error> {
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
    2 => {
      run_file(&mut vm, &args[1]).unwrap_or_else(|error| match error {
        Error::VirtualMachineError(VirtualMachineError::InterpreterError(_)) => {
          // println!("{:#?}", error);
          exit(65);
        },
        Error::VirtualMachineError(VirtualMachineError::RuntimeError(_)) => {
          // println!("{:#?}", error);
          exit(70);
        },
        _ => {
          eprintln!("{:#?}", error);
          exit(144);
        },
      });
      Ok(())
    },
    _ => exit(-1),
  }
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }

  #[test]
  #[should_panic]
  pub fn test() {
    init();

    let mut vm = VirtualMachine::new();
    run_file(&mut vm, "nonexistent file.txt").unwrap();
  }

  #[test]
  pub fn test2() {
    init();

    let mut vm = VirtualMachine::new();
    let output = Vec::new();
    let input = b"3 + 4";
    repl(&mut vm, &input[..], output).unwrap();
  }
}
