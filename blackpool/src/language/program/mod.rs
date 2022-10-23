use std::error::Error as StdError;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use crate::language::instructions::Instructions;

/// A program consisting of bytecode.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Program {
  /// The serialized instructions comprising the program.
  pub instructions: Instructions,
}

impl Program {
  /// Constructor.
  #[named]
  pub fn new(instructions: Instructions) -> Self {
    trace_enter!();
    let result = Program { instructions };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Dump the disassembled program to a std::fmt::Write object.
  #[named]
  pub fn dump_fmt<W: FmtWrite>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    trace_enter!();
    self.instructions.dump(out)?;
    trace_exit!();
    Ok(())
  }

  /// Dump the disassembled program to a std::io::Write object.
  #[named]
  pub fn dump_io<W: IoWrite>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    trace_enter!();
    let mut string = String::new();
    self.dump_fmt(&mut string)?;
    write!(out, "{}", string)?;
    trace_exit!();
    Ok(())
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_default() {
    init();
    trace_enter!();
    trace_exit!();
  }
}
