use std::error::Error as StdError;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use crate::scripting_language::constants::Constants;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::instructions::Instructions;

/// A program consisting of bytecode.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Program {
  /// The serialized instructions comprising the program.
  pub instructions: Instructions,
  /// The constant pool.
  pub constants: Constants,
}

impl Program {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let instructions = Instructions::default();
    let constants = Constants::default();
    let result = Program {
      instructions,
      constants,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Append an instruction to the program.
  #[named]
  pub fn append_instruction(&mut self, instruction: Instruction, line_number: usize) {
    trace_enter!();
    trace_var!(instruction);
    trace_var!(line_number);
    self.instructions.append(instruction, line_number);
    trace_exit!();
  }

  /// Dump the disassembled program to a std::fmt::Write object.
  #[named]
  #[inline]
  pub fn dump_fmt<W: FmtWrite>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    trace_enter!();
    self.instructions.dump(out)?;
    trace_exit!();
    Ok(())
  }

  /// Dump the disassembled program to a std::io::Write object.
  #[named]
  #[inline]
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
