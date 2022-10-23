use std::error::Error as StdError;
use std::fmt::Write;

use crate::scripting_language::instruction::Instruction;

/// The `Instructions` collection.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Instructions {
  /// The actual collected instructions.
  pub instructions: Vec<Instruction>,
  /// The line numbers corresponding to the instructions.
  pub line_numbers: Vec<usize>,
}

impl Instructions {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let instructions = Vec::new();
    trace_var!(instructions);
    let line_numbers = Vec::new();
    trace_var!(line_numbers);
    let result = Self {
      instructions,
      line_numbers,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Append an instruction to the program.
  #[named]
  pub fn append(&mut self, instruction: Instruction, line_number: usize) {
    trace_enter!();
    trace_var!(instruction);
    trace_var!(line_number);
    assert_eq!(self.instructions.len(), self.line_numbers.len());
    self.instructions.push(instruction);
    self.line_numbers.push(line_number);
    trace_exit!();
  }

  /// Dump the disassembled memory.
  #[named]
  pub fn dump<W: Write>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    trace_enter!();
    writeln!(out)?;
    writeln!(
      out,
      "{:6}  {:>6}  {:>6}  {:>16}  {:>4}",
      "Index", "Offset", "Line", "Instruction", "Args"
    )?;
    writeln!(out, "--------------------------------------")?;
    let mut offset = 0;
    for (index, instruction) in self.instructions.iter().enumerate() {
      // Add one for the byte width of the operand.
      offset += instruction.dump(index, offset, self.line_numbers[index], out)? + 1;
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
    let mut instructions = Instructions::default();
    instructions.append(instruction, 1);
    let result = instructions.dump(&mut string).unwrap();
    assert_eq!(result, ());
    assert_eq!(string, "\nIndex   Offset    Line       Instruction  Args\n--------------------------------------\n    0   0x0000       1            Return     0\n");
    trace_exit!();
  }
}
