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
  /// Append an instruction to the chunk.
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
    writeln!(out, "----------------------------------------------")?;
    let mut offset = 0;
    for (index, instruction) in self.instructions.iter().enumerate() {
      // Add one for the byte width of the operand.
      let line = self.line_numbers[index];
      let same_line = index > 0 && self.line_numbers[index] == self.line_numbers[index - 1];
      offset += instruction.dump(index, offset, line, same_line, out)? + 1;
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
    println!("{}", string);
    let lines: Vec<&str> = string.split("\n").collect();
    assert_eq!(lines[0], "");
    assert_eq!(lines[1], "Index   Offset    Line       Instruction  Args");
    assert_eq!(lines[2], "----------------------------------------------");
    assert_eq!(lines[3], "    0   0x0000       1            Return     0");
    trace_exit!();
  }
}
