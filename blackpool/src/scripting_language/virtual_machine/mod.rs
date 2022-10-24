use crate::scripting_language::compiler::Compiler;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::program::Program;
use crate::scripting_language::value::Value;

pub mod constants;
use constants::*;
pub mod error;
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

  /// Interpret some source code.
  #[named]
  pub fn interpret(&mut self, source: &String) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    self.compile(source)?;
    trace_exit!();
    Ok(())
  }

  /// Compile the source code.
  #[named]
  pub fn compile(&mut self, source: &String) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    let mut compiler = Compiler::default();
    trace_var!(compiler);
    let mut program = Program::default();
    compiler.compile(source, &mut program)?;
    trace_var!(program);
    self.run(&program)?;
    println!("{:#?}", program);
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
      use Value::*;
      match instruction {
        Constant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
          self.push(constant)?;
        },
        LongConstant(index) => {
          let constant = program.constants.constants[index as usize];
          trace_var!(constant);
          self.push(constant)?;
        },
        Negate => {
          let pop = self.pop()?;
          trace_var!(pop);
          match pop {
            Number(pop) => {
              let answer = -pop;
              trace_var!(answer);
              self.push(Value::Number(answer))?;
            },
            _ => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperand(Negate, pop)));
            },
          }
        },
        Add => {
          let addend = self.pop()?;
          trace_var!(addend);
          let augend = self.pop()?;
          trace_var!(augend);
          match (addend, augend) {
            (Number(a), Number(b)) => self.push(Value::Number(b + a))?,
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                Add, augend, addend,
              )))
            },
          }
        },
        Subtract => {
          let subtrahend = self.pop()?;
          trace_var!(subtrahend);
          let minuend = self.pop()?;
          trace_var!(minuend);
          match (subtrahend, minuend) {
            (Number(a), Number(b)) => self.push(Value::Number(b - a))?,
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                Subtract, minuend, subtrahend,
              )))
            },
          }
        },
        Multiply => {
          let multiplier = self.pop()?;
          trace_var!(multiplier);
          let multiplicand = self.pop()?;
          trace_var!(multiplicand);
          match (multiplier, multiplicand) {
            (Number(a), Number(b)) => self.push(Value::Number(b * a))?,
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                Multiply,
                multiplicand,
                multiplier,
              )))
            },
          }
        },
        Divide => {
          let divisor = self.pop()?;
          trace_var!(divisor);
          let dividend = self.pop()?;
          trace_var!(dividend);
          match (divisor, dividend) {
            (Number(a), Number(b)) => self.push(Value::Number(b / a))?,
            (_, _) => {
              return Err(Error::RuntimeError(RuntimeError::InappropriateOperands(
                Divide, dividend, divisor,
              )))
            },
          }
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
    test_instructions!([Return], [] => []);
    test_instructions!([Return], [Number(53.0)] => [Number(53.0)]);
    test_instructions!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_instructions!([Negate], [Number(-53.0)] => [Number(53.0)]);
    test_instructions!([Add], [Number(-53.0), Number(4.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(4.0), Number(-53.0)] => [Number(-49.0)]);
    trace_exit!();
  }
}
