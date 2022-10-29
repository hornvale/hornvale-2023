use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

/// The `Instruction` type.
///
/// An instruction consists of an opcode and its arguments.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Instruction {
  /// Produce a particular constant (16-bit operand length).
  Constant(u16),
  /// Define a particular global.
  DefineGlobal(u16),
  /// Set a particular global.
  SetGlobal(u16),
  /// Get a particular global.
  GetGlobal(u16),
  /// Set a particular local.
  SetLocal(u16),
  /// Get a particular local.
  GetLocal(u16),
  /// Close an upvalue.
  CloseUpvalue,
  /// Set a particular upvalue.
  SetUpvalue(u16),
  /// Get a particular upvalue.
  GetUpvalue(u16),
  /// Jump unconditionally.
  Jump(u16),
  /// Jump if the top value on the stack is falsey.
  JumpIfFalse(u16),
  /// Loop back to the offset indicated by the top value on the stack.
  Loop(u16),
  /// Reference to a closure.
  Closure(u16),
  /// A function call and the number of arguments.
  Call(u8),
  /// A class declaration and index of its name.
  Class(u16),
  /// A method declaration and index of its name.
  Method(u16),
  /// Get an instance property.
  GetProperty(u16),
  /// Set an instance property.
  SetProperty(u16),
  /// Invoke a method call with the specified name and argument count.
  Invoke((u16, u8)),
  /// Unary negate operation, performed on the top of the stack.
  Negate,
  /// Add the two values on the top of the stack.
  Add,
  /// Subtract the two values on the top of the stack.
  Subtract,
  /// Multiply the two values on the top of the stack.
  Multiply,
  /// Divide the two values on the top of the stack.
  Divide,
  /// Return whence we came!
  Return,
  /// Push Nil onto the stack.
  Nil,
  /// Push True onto the stack.
  True,
  /// Push False onto the stack.
  False,
  /// Boolean opposite of the top of the stack.
  Not,
  /// Equality for the top two values of the stack.
  Equal,
  /// Not-Equality for the top two values of the stack.
  NotEqual,
  /// GreaterThan for the top two values of the stack.
  GreaterThan,
  /// LessThan for the top two values of the stack.
  LessThan,
  /// GreaterThanOrEqual for the top two values of the stack.
  GreaterThanOrEqual,
  /// LessThanOrEqual for the top two values of the stack.
  LessThanOrEqual,
  /// Print the top value on the stack.
  Print,
  /// Pop the value off the stack.
  Pop,
  /// Inherit from a superclass.
  Inherit,
}

impl Instruction {
  /// Show the disassembled version of the instruction.
  #[named]
  pub fn dump<W: Write>(
    &self,
    index: usize,
    offset: usize,
    line: usize,
    same_line: bool,
    out: &mut W,
  ) -> Result<usize, Box<dyn StdError>> {
    trace_enter!();
    trace_var!(index);
    trace_var!(offset);
    trace_var!(line);
    use Instruction::*;
    let result = match &self {
      Constant(_) | DefineGlobal(_) | SetGlobal(_) | GetGlobal(_) => 1,
      _ => 0,
    };
    let line_string = match same_line {
      true => "|".to_string(),
      false => line.to_string(),
    };
    #[allow(clippy::to_string_in_format_args)]
    writeln!(
      out,
      "{:5}   {:#06X}  {:>6}  {:>16}  {:>4}",
      index,
      offset,
      line_string,
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
    assert_eq!(Instruction::Negate.to_string(), "Negate");
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
    let result = Instruction::Return.dump(0, 0, 0, false, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "    0   0x0000       0            Return     0\n");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test2() {
    init();
    trace_enter!();
    let mut string = String::new();
    let result = Instruction::Return.dump(16, 23, 72, false, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "   16   0x0017      72            Return     0\n");
    trace_exit!();
  }

  #[named]
  #[test]
  pub fn test3() {
    init();
    trace_enter!();
    let mut string = String::new();
    let result = Instruction::Return.dump(16, 23, 72, true, &mut string).unwrap();
    assert_eq!(result, 0);
    assert_eq!(string, "   16   0x0017       |            Return     0\n");
    trace_exit!();
  }
}
