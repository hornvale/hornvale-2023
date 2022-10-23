use std::error::Error as StdError;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use crate::scripting_language::constants::Constants;
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
  use crate::scripting_language::instruction::Instruction;
  use crate::scripting_language::value::Value;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test() {
    init();
    trace_enter!();
    let mut string = String::new();
    let mut program = Program::default();
    let const_inst = program.constants.push(Value::Number(1.2)).unwrap();
    program.instructions.append(const_inst, 1);
    program.instructions.append(Instruction::Return, 2);
    let result = program.instructions.dump(&mut string).unwrap();
    assert_eq!(result, ());
    let lines: Vec<&str> = string.split("\n").collect();
    assert_eq!(lines[0], "");
    assert_eq!(lines[1], "Index   Offset    Line       Instruction  Args");
    assert_eq!(lines[2], "----------------------------------------------");
    assert_eq!(lines[3], "    0   0x0000       1       Constant(0)     1");
    assert_eq!(lines[4], "    1   0x0002       2            Return     0");
    trace_exit!();
  }
}
