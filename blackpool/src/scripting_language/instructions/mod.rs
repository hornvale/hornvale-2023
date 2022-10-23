use std::error::Error as StdError;
use std::fmt::Write;

use crate::scripting_language::instruction::Instruction;

/// The `Instructions` collection.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Instructions {
  pub instructions: Vec<Instruction>,
}

impl Instructions {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let instructions = Vec::new();
    trace_var!(instructions);
    let result = Self { instructions };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Dump the disassembled memory.
  #[named]
  pub fn dump<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    trace_enter!();
    writeln!(out)?;
    writeln!(
      out,
      "{:6}  {:>6}  {:>16}  {:>4}",
      "Index", "Offset", "Instruction", "Args"
    )?;
    writeln!(out, "--------------------------------------")?;
    let mut offset = 0;
    for (index, instruction) in self.instructions.iter().enumerate() {
      // Add one for the byte width of the operand.
      offset += instruction.dump(index, offset, out)? + 1;
    }
    writeln!(out)?;
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
  pub fn test() {
    init();
    trace_enter!();
    let mut string = String::new();
    let instruction = Instruction::Return;
    let instructions = Instructions {
      instructions: vec![instruction],
    };
    let result = instructions.dump(&mut string).unwrap();
    assert_eq!(result, ());
    assert_eq!(string, "\nIndex   Offset       Instruction  Args\n--------------------------------------\n    0   0x0000            Return     0\n");
    trace_exit!();
  }
}
