use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use crate::scripting_language::constants::ConstantsIndexType;

/// The `Instruction` type.
///
/// An instruction consists of an opcode and its arguments.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Instruction {
  /// Produce a particular constant.
  Constant(ConstantsIndexType),
  /// Return whence we came!
  Return,
}

impl Instruction {
  /// Show the disassembled version of the instruction.
  #[named]
  pub fn dump<W: Write>(
    &self,
    index: usize,
    offset: usize,
    line: usize,
    out: &mut W,
  ) -> Result<usize, Box<dyn StdError>> {
    trace_enter!();
    trace_var!(index);
    trace_var!(offset);
    trace_var!(line);
    use Instruction::*;
    let result = match &self {
      Constant(_) => 1,
      _ => 0,
    };
    write!(
      out,
      "{:>5}   {:#06X}  {:>6}  {:>16}  {:>4}",
      index, offset, line, self, result,
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
    assert_eq!(Instruction::Constant(5).to_string(), "Constant(5)");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test() {
    init();
    trace_enter!();
    let mut string = String::new();
    let result = Instruction::Return.dump(0, 0, 0, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "    0   0x0000       0            Return     0");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test2() {
    init();
    trace_enter!();
    let mut string = String::new();
    let result = Instruction::Return.dump(16, 23, 72, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "   16   0x0017      72            Return     0");
    trace_exit!();
  }
}
