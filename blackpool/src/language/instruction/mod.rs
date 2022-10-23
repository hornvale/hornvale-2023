use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

/// The `Instruction` type.
///
/// An instruction consists of an opcode and its arguments.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Instruction {
  Return,
}

impl Instruction {
  /// Show the disassembled version of the instruction.
  #[named]
  pub fn dump<W: Write>(&self, index: usize, offset: usize, out: &mut W) -> Result<usize, Box<dyn StdError>> {
    trace_enter!();
    let arguments = 0;
    use Instruction::*;
    let result = match &self {
      _ => 0,
    };
    write!(
      out,
      "{:>5}   {:#06X}  {:>16}  {:>4}",
      index,
      offset,
      self.to_string(),
      result,
    )?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Display for Instruction {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", self)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_fmt() {
    init();
    trace_enter!();
    assert_eq!(Instruction::Return.to_string(), "Return");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test() {
    init();
    trace_enter!();
    let mut string = String::new();
    let instruction = Instruction::Return;
    let result = instruction.dump(0, 0, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "    0   0x0000            Return     0");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test2() {
    init();
    trace_enter!();
    let mut string = String::new();
    let instruction = Instruction::Return;
    let result = instruction.dump(16, 23, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "   16   0x0017            Return     0");
    trace_exit!();
  }
}
