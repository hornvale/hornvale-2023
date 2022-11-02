use std::error::Error as StdError;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use crate::scripting_language::constants::Constants;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::instructions::Instructions;
use crate::scripting_language::value::Value;

/// A chunk or portion thereof consisting of bytecode.
#[derive(Clone, Debug, Default, Display)]
#[display(fmt = "instructions: {}, constants: {}", instructions, constants)]
pub struct Chunk {
  /// The serialized instructions comprising the chunk.
  pub instructions: Instructions,
  /// The constant pool.
  pub constants: Constants,
}

impl Chunk {
  /// Dump the disassembled chunk to a std::fmt::Write object.
  #[inline]
  pub fn dump_fmt<W: FmtWrite>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    self.instructions.dump(out)?;

    Ok(())
  }

  /// Dump the disassembled chunk to a std::io::Write object.
  #[inline]
  pub fn dump_io<W: IoWrite>(&self, out: &mut W) -> Result<(), Box<dyn StdError>> {
    let mut string = String::new();
    self.dump_fmt(&mut string)?;
    write!(out, "{}", string)?;

    Ok(())
  }

  /// Read a string.
  pub fn read_string(&self, index: u16) -> Reference<String> {
    if let Value::String(string) = self.constants.constants[index as usize] {
      string
    } else {
      panic!("Constant is not String!")
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::scripting_language::instruction::Instruction;
  use crate::scripting_language::value::Value;
  use crate::test::*;

  #[test]
  pub fn test() {
    init();
    let mut string = String::new();
    let mut chunk = Chunk::default();
    let const_inst = chunk.constants.push(Value::Number(1.2)).unwrap();
    chunk.instructions.append(const_inst, 1);
    chunk.instructions.append(Instruction::Return, 2);
    let result = chunk.dump_fmt(&mut string).unwrap();
    assert_eq!(result, ());
    let lines: Vec<&str> = string.split("\n").collect();
    assert_eq!(lines[0], "");
    assert_eq!(lines[1], "Index   Offset    Line       Instruction  Args");
    assert_eq!(lines[2], "----------------------------------------------");
    assert_eq!(lines[3], "    0   0x0000       1       Constant(0)     1");
    assert_eq!(lines[4], "    1   0x0002       2            Return     0");
  }
}
