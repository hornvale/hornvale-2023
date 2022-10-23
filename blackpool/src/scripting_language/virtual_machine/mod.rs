use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::program::Program;
use crate::scripting_language::value::Value;

pub mod constants;
use constants::*;
pub mod error;
// use error::compilation::CompilationError;
use error::runtime::RuntimeError;
use error::Error;

/// The `VirtualMachine` type.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VirtualMachine {
  /// Points at the next instruction to read.
  pub instruction_pointer: usize,
  /// The stack.
  pub stack: Vec<Value>,
}

impl VirtualMachine {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let instruction_pointer = 0;
    trace_var!(instruction_pointer);
    let stack = Vec::with_capacity(STACK_SIZE_MAX);
    trace_var!(stack);
    let result = Self {
      instruction_pointer,
      stack,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Interpret a program.
  #[named]
  pub fn interpret(&mut self, program: &Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.run(program)?;
    trace_exit!();
    Ok(())
  }

  /// Run the program.
  #[named]
  pub fn run(&mut self, program: &Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    loop {
      let instruction_option = program.instructions.instructions.get(self.instruction_pointer);
      if instruction_option.is_none() {
        break;
      }
      let instruction = *instruction_option.unwrap();
      trace_var!(instruction);
      use Instruction::*;
      match instruction {
        Constant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
        },
        LongConstant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
        },
        Negate => {
          let pop = self.pop()?;
          self.push(-pop)?;
        },
        Return => break,
      }
      self.instruction_pointer += 1;
    }
    trace_exit!();
    Ok(())
  }

  /// Push a value to the stack.
  #[named]
  #[inline]
  pub fn push(&mut self, value: Value) -> Result<(), Error> {
    trace_enter!();
    trace_var!(value);
    if self.stack.len() > STACK_SIZE_MAX {
      return Err(Error::RuntimeError(RuntimeError::StackOverflow));
    }
    trace_var!(self.stack);
    self.stack.push(value);
    trace_exit!();
    Ok(())
  }

  /// Push a value to the stack.
  #[named]
  #[inline]
  pub fn pop(&mut self) -> Result<Value, Error> {
    trace_enter!();
    if self.stack.is_empty() {
      return Err(Error::RuntimeError(RuntimeError::StackUnderflow));
    }
    trace_var!(self.stack);
    let result = self.stack.pop().unwrap();
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_vm() {
    init();
    trace_enter!();
    use crate::scripting_language::value::Value::*;
    test_program!([Return], [] => []);
    test_program!([Return], [Number(53.0)] => [Number(53.0)]);
    test_program!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_program!([Negate], [Number(-53.0)] => [Number(53.0)]);
    trace_exit!();
  }
}
